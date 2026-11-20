# Mobile 3D Studio

A mobile-first, browser-based 3D scene editor and animation prototype.

## Run it
Open the hosted site in a modern browser with WebGL enabled. The renderer currently loads Three.js from jsDelivr, so an internet connection is required on first load.

## Current features
- Touch orbit and pinch zoom
- Add box, sphere, cylinder, cone and torus objects
- Select objects from the scene list or tap them in the viewport
- Duplicate and delete objects
- Parent objects to create a transform hierarchy; unparent while preserving world position
- Indented scene list and parent selector
- Touch-friendly X/Y/Z position, rotation and scale buttons
- Material colour, roughness and metalness controls
- Object transform keyframes on a 10-second timeline
- Camera position/target keyframes, shown separately on the timeline
- Playback interpolates object transforms and camera movement together
- Save scenes locally on the device, including parent-child relationships
- Export scene JSON and import it again later
- Basic installable PWA shell with an app icon and cache refresh

## Known limitations
Parenting should be set up before adding animation keyframes to the child object. The current animation system stores local transforms and does not yet retarget existing child keyframes when its parent changes.

This is an evolving foundation, not Blender. Animation currently covers transforms and camera movement; there is no character rigging, mesh sculpting, audio, video rendering, cloud sync or advanced graph editor yet. The Three.js library is loaded from a CDN, so full offline use is not guaranteed.

## Next priorities
1. Add multi-object selection and collection/group management.
2. Build mesh Edit Mode with vertex, edge and face selection.
3. Add core modelling operations such as extrude, inset and bevel.
4. Add modifiers and reusable scene assets.
5. Build rigging and character animation tools.

## Hosting
GitHub Pages publishes the main branch from the repository root.
