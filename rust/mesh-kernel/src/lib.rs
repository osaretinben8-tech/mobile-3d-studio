use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh {
    pub vertices: Vec<[f64; 3]>,
    pub faces: Vec<Vec<usize>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MeshReport {
    pub vertices: usize,
    pub faces: usize,
    pub edges: usize,
    pub boundary_edges: usize,
    pub non_manifold_edges: usize,
    pub degenerate_faces: usize,
    pub connected_components: usize,
    pub valid: bool,
    pub issues: Vec<String>,
}

fn sub(a: [f64;3], b: [f64;3]) -> [f64;3] { [a[0]-b[0],a[1]-b[1],a[2]-b[2]] }
fn add(a: [f64;3], b: [f64;3]) -> [f64;3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn mul(a: [f64;3], s: f64) -> [f64;3] { [a[0]*s,a[1]*s,a[2]*s] }
fn dot(a: [f64;3], b: [f64;3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a: [f64;3], b: [f64;3]) -> [f64;3] {
    [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
}
fn length(a: [f64;3]) -> f64 { dot(a,a).sqrt() }
fn normalized(a: [f64;3]) -> [f64;3] { let l=length(a); if l<1e-12 {[0.0,1.0,0.0]} else {mul(a,1.0/l)} }
fn edge_key(a: usize,b: usize)->(usize,usize){if a<b{(a,b)}else{(b,a)}}

pub fn validate(mesh: &Mesh) -> Result<MeshReport,String> {
    if mesh.vertices.len()<3 || mesh.faces.is_empty(){return Err("mesh needs at least 3 vertices and 1 face".into())}
    if mesh.vertices.iter().flatten().any(|v|!v.is_finite() || v.abs()>1.0e6){return Err("vertex coordinates must be finite and within ±1,000,000".into())}
    let mut issues=Vec::new();
    let mut edges=std::collections::BTreeMap::<(usize,usize),usize>::new();
    let mut degenerate=0;
    let mut adjacency=vec![Vec::<usize>::new();mesh.vertices.len()];
    for (fi,face) in mesh.faces.iter().enumerate(){
        if face.len()<3 {issues.push(format!("face {fi} has fewer than 3 corners"));continue}
        let mut unique=std::collections::BTreeSet::new();
        for &i in face {if i>=mesh.vertices.len(){return Err(format!("face {fi} references missing vertex {i}"))} unique.insert(i);}
        if unique.len()!=face.len(){issues.push(format!("face {fi} repeats a vertex"));continue}
        let mut area=[0.0;3];
        for k in 0..face.len(){
            let a=face[k];let b=face[(k+1)%face.len()];
            *edges.entry(edge_key(a,b)).or_insert(0)+=1;
            adjacency[a].push(b);adjacency[b].push(a);
            area=add(area,cross(mesh.vertices[a],mesh.vertices[b]));
        }
        if length(area)<1e-10{degenerate+=1;issues.push(format!("face {fi} is degenerate"))}
    }
    let boundary=edges.values().filter(|&&n|n==1).count();
    let nonmanifold=edges.values().filter(|&&n|n>2).count();
    if nonmanifold>0{issues.push(format!("{nonmanifold} edge(s) belong to more than two faces"))}
    let mut seen=vec![false;mesh.vertices.len()];let mut components=0;
    for start in 0..mesh.vertices.len(){if seen[start]||adjacency[start].is_empty(){continue}components+=1;let mut stack=vec![start];seen[start]=true;while let Some(v)=stack.pop(){for &n in &adjacency[v]{if !seen[n]{seen[n]=true;stack.push(n)}}}}
    Ok(MeshReport{vertices:mesh.vertices.len(),faces:mesh.faces.len(),edges:edges.len(),boundary_edges:boundary,non_manifold_edges:nonmanifold,degenerate_faces:degenerate,connected_components:components,valid:issues.is_empty(),issues})
}

fn face_normal(mesh:&Mesh,face:&[usize])->Result<[f64;3],String>{
    if face.len()<3{return Err("selected face has fewer than 3 vertices".into())}
    let mut n=[0.0;3];
    for i in 0..face.len(){let a=*mesh.vertices.get(face[i]).ok_or("face vertex is out of range")?;let b=*mesh.vertices.get(face[(i+1)%face.len()]).ok_or("face vertex is out of range")?;n=add(n,cross(a,b));}
    if length(n)<1e-10{return Err("selected face has no usable surface normal".into())}
    Ok(normalized(n))
}

pub fn extrude_face(mesh:&Mesh, face_index:usize, distance:f64)->Result<Mesh,String>{
    if !distance.is_finite()||distance.abs()<1e-8||distance.abs()>1000.0{return Err("extrusion distance must be between 0.00000001 and 1000".into())}
    let face=mesh.faces.get(face_index).ok_or("selected face index is out of range")?;
    let normal=face_normal(mesh,face)?;
    let mut out=mesh.clone();out.faces.remove(face_index);let mut top=Vec::with_capacity(face.len());
    for &idx in face {let p=add(mesh.vertices[idx],mul(normal,distance));top.push(out.vertices.len());out.vertices.push(p);}
    out.faces.push(top.clone());
    for i in 0..face.len(){let j=(i+1)%face.len();out.faces.push(vec![face[i],face[j],top[j],top[i]]);}
    validate(&out)?;
    Ok(out)
}

fn project(p:[f64;3],axis:usize)->[f64;2]{match axis{0=>[p[1],p[2]],1=>[p[0],p[2]],_=>[p[0],p[1]]}}
fn area2(points:&[[f64;2]])->f64{let mut a=0.0;for i in 0..points.len(){let p=points[i];let q=points[(i+1)%points.len()];a+=p[0]*q[1]-q[0]*p[1]}a*0.5}
fn orient(a:[f64;2],b:[f64;2],c:[f64;2])->f64{(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])}
fn inside(p:[f64;2],a:[f64;2],b:[f64;2],c:[f64;2],sign:f64)->bool{
    orient(a,b,p)*sign>=-1e-10 && orient(b,c,p)*sign>=-1e-10 && orient(c,a,p)*sign>=-1e-10
}
pub fn triangulate_face(mesh:&Mesh,face_index:usize)->Result<Vec<[usize;3]>,String>{
    let face=mesh.faces.get(face_index).ok_or("selected face index is out of range")?;
    if face.len()==3{return Ok(vec![[face[0],face[1],face[2]]])}
    let n=face_normal(mesh,face)?;
    let axis=if n[0].abs()>n[1].abs()&&n[0].abs()>n[2].abs(){0}else if n[1].abs()>n[2].abs(){1}else{2};
    let points:Vec<[f64;2]>=face.iter().map(|&i|project(mesh.vertices[i],axis)).collect();
    let sign=if area2(&points)>=0.0{1.0}else{-1.0};
    let mut remaining:Vec<usize>=(0..face.len()).collect();let mut tris=Vec::new();let mut guard=0;
    while remaining.len()>3 && guard<face.len()*face.len(){guard+=1;let mut found=false;
        for i in 0..remaining.len(){let a=remaining[(i+remaining.len()-1)%remaining.len()];let b=remaining[i];let c=remaining[(i+1)%remaining.len()];
            if orient(points[a],points[b],points[c])*sign<=1e-10{continue}
            if remaining.iter().copied().any(|p|p!=a&&p!=b&&p!=c&&inside(points[p],points[a],points[b],points[c],sign)){continue}
            tris.push([face[a],face[b],face[c]]);remaining.remove(i);found=true;break
        }
        if !found{return Err("face is self-intersecting or too degenerate to triangulate".into())}
    }
    if remaining.len()==3{tris.push([face[remaining[0]],face[remaining[1]],face[remaining[2]]]);}
    if tris.len()!=face.len()-2{return Err("triangulation did not produce a complete face".into())}
    Ok(tris)
}

#[derive(Deserialize)]
#[serde(tag="operation",rename_all="snake_case")]
enum Request { Inspect{mesh:Mesh}, Extrude{mesh:Mesh,face:usize,distance:f64}, Triangulate{mesh:Mesh,face:usize} }

#[wasm_bindgen]
pub fn process_mesh_json(input:&str)->Result<String,JsValue>{
    let request:Request=serde_json::from_str(input).map_err(|e|JsValue::from_str(&format!("invalid mesh request: {e}")))?;
    let result=match request{
        Request::Inspect{mesh}=>serde_json::to_string(&validate(&mesh).map_err(|e|JsValue::from_str(&e))?),
        Request::Extrude{mesh,face,distance}=>serde_json::to_string(&extrude_face(&mesh,face,distance).map_err(|e|JsValue::from_str(&e))?),
        Request::Triangulate{mesh,face}=>serde_json::to_string(&triangulate_face(&mesh,face).map_err(|e|JsValue::from_str(&e))?),
    }.map_err(|e|JsValue::from_str(&e.to_string()))?;
    Ok(result)
}

#[cfg(test)]
mod tests{
    use super::*;
    fn quad()->Mesh{Mesh{vertices:vec![[0.,0.,0.],[1.,0.,0.],[1.,1.,0.],[0.,1.,0.]],faces:vec![vec![0,1,2,3]]}}
    #[test]fn validates_quad(){let r=validate(&quad()).unwrap();assert_eq!(r.edges,4);assert_eq!(r.boundary_edges,4);assert!(r.valid)}
    #[test]fn triangulates_quad(){let t=triangulate_face(&quad(),0).unwrap();assert_eq!(t.len(),2)}
    #[test]fn extrudes_quad_as_connected_prism(){let m=extrude_face(&quad(),0,0.5).unwrap();assert_eq!(m.vertices.len(),8);assert_eq!(m.faces.len(),5);assert_eq!(validate(&m).unwrap().non_manifold_edges,0)}
    #[test]fn rejects_invalid_index(){let m=Mesh{vertices:vec![[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],faces:vec![vec![0,1,9]]};assert!(validate(&m).is_err())}
}
