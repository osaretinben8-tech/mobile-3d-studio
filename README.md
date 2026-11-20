# Mobile 3D Studio

A mobile-first, browser-based 3D scene editor and animation prototype.

## Run it
Open the hosted site in a modern browser with WebGL enabled. The renderer currently loads Three.js from jsDelivr, so an internet connection is required on first load.

## Current features
- Touch orbit and pinch zoom
- Add box, sphere, cylinder, cone and torus objects
- Select objects from the scene list or tap them in the viewport
- Duplicate and delete objects
- Touch-friendly X/Y/Z position, rotation and scale buttons
- Material colour, roughness and metalness controls
- Object transform keyframes on a 10-second timeline
- Camera position/target keyframes, shown separately on the timeline
- Playback interpolates object transforms and camera movement together
- Save scenes locally on the device
- Export scene JSON and import it again later
- Basic installable PWA shell with an app icon and cache refresh

## Known limitations
This is an evolving foundation, not Blender. Animation currently covers transforms and camera movement; there is no character rigging, mesh sculpting, audio, video rendering, cloud sync or advanced graph editor yet. The Three.js library is loaded from a CDN, so full offline use is not guaranteed.

## Next priorities
1. Test camera and scene import workflows on real Android devices.
2. Improve touch gestures and object selection.
3. Add animation easing and distinct per-object tracks.
4. Add camera lens controls, lights and reusable scene assets.
5. Profile performance on lower-memory phones.

## Hosting
GitHub Pages publishes the main branch from the repository root.
