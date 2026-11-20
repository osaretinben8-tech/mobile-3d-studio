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
- Keyframes, a 10-second timeline and playback
- Save a scene locally on the device and export scene data as JSON
- Basic PWA shell caching

## Known limitations
This is an early foundation, not Blender. The current animation system interpolates transforms only. There is no character rigging, mesh modelling, audio, video rendering or project cloud sync yet. PWA offline support caches the app shell, but the Three.js library is loaded from a CDN and may still require internet.

## Next priorities
1. Verify on real Android touch devices and improve selection/gesture handling.
2. Add import/export that fully restores scene data.
3. Add camera keyframes, multiple tracks and animation easing.
4. Add primitive editing, lights and reusable scene assets.
5. Profile performance on lower-memory phones.

## Hosting
Enable GitHub Pages for the main branch and root folder in repository Settings, then Pages. The site will then be available at the GitHub Pages URL for this repository.
