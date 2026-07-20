# BRAINSTORMING

A non-Newtonian fluid is a substance that changes its viscosity (thickness) depending on the amount of force or stress applied. Unlike water (which is Newtonian and flows consistently), these fluids can instantly turn solid when struck or become much runnier when stirred. [1, 2, 3, 4]

## Types of Behavior

Non-Newtonian fluids generally fall into distinct categories based on how they react to force: [5]

* Shear-Thickening (Dilatant): These become thicker and more solid when sudden force is applied. A classic example is Oobleck, a mixture of cornstarch and water. If you slowly dip your finger into it, it acts like a liquid, but if you punch it, it resists like a solid. [2, 3, 4, 6]
* Shear-Thinning (Pseudoplastic): These become thinner and flow more easily under stress. Common examples include ketchup, toothpaste, and paint. Ketchup gets stuck in the bottle, but shaking it applies force, causing the fluid to thin out and pour smoothly. [6, 7, 8]
* Time-Dependent Fluids: These fluids change their viscosity not just based on force, but on how long the force is applied. Thixotropic fluids (like honey or certain paints) get runnier the longer you stir them, while rheopectic fluids (like cream) get thicker the longer you whip them. [5, 6, 9]

## Common Everyday Examples

You likely interact with several non-Newtonian fluids every single day: [7, 10]

* Blood: Acts as a shear-thinning fluid, which helps it flow smoothly through your blood vessels.
* Toothpaste: Stays firm on your toothbrush, but easily squeezes out of the tube.
* Quicksand: Behaves like a solid until you struggle or apply sudden pressure, which causes it to act more like a liquid, potentially trapping objects. [5, 7]

## How to Make Oobleck (Shear-Thickening)

You can easily create and test a shear-thickening fluid at home using basic kitchen supplies. [2, 3]

   1. Combine $1 \frac{1}{2}$ cups of cornstarch with 1 cup of water in a large bowl.
   2. Stir slowly until the mixture reaches the consistency of honey.
   3. Scoop it up and try rolling it into a ball between your palms, then stop squeezing and watch it flow out like a liquid. [3, 11]

· 1970 M01 1

---
USER:
> im looking for the opposite. i know it doesnt exist, im just wondering if there is a formal name or definition. im looking to describe an element that holds is shape by default, then depending on force applied its shape softens, if enough, liquified. The latter part is what i am talking about when i say "opposite of non nutonian".

---

The substance you are describing actually does exist in the real world.
In physics and rheology, the exact term for a material that holds its shape like a solid by default, but turns into a liquid when enough force is applied, is a Bingham plastic (or a yield stress fluid).
Because it liquefies under stress, it is a specific type of shear-thinning material, which is still classified under the umbrella of non-Newtonian fluids.

## The Formal Mechanics

* Yield Stress: This is the critical threshold of force required to make the material move. Below this threshold, the material behaves like a solid elastic sheet. Above this threshold, the internal bonds break, and it flows like a liquid.
* The "Opposite" Logic: You are thinking of it as the opposite of Oobleck (shear-thickening, which goes from liquid to solid). In that context, your intuition is completely correct: shear-thinning is the behavioral opposite of shear-thickening. [12, 13]

## Everyday Examples

You likely use these "solid-to-liquid" materials every day without realizing it:

* Toothpaste: It sits perfectly still on your toothbrush like a solid. However, once you apply force by squeezing the tube, it exceeds its yield stress and flows out like a liquid. [14]

* Mayonnaise and Ketchup: You can pile mayonnaise into a peak on a sandwich, and it holds its shape. When you spread it with a knife (applying force), it instantly liquefies to coat the bread.
* Clay and Mud: Pottery clay holds its molded shape until a sculptor applies enough physical force to make it pliable and fluid.

---

JPEG XL (often abbreviated as JXL), which is frequently discussed alongside WebP and AVIF. [24]

## What is JPEG XL?

JPEG XL (.jxl) is an advanced, next-generation image format designed to completely replace the original 1992 JPEG standard. It is highly regarded by web developers for its extreme efficiency and unique feature set. [24, 25, 26]

## Why Developers love JPEG XL (The Features)

* Lossless JPEG Transcoding: It can shrink existing JPEGs by roughly 20% completely losslessly, and reverse the process back to the exact original file bit-for-bit. [27, 28]
* Insane Lossless Compression: It outperforms WebP and PNG when saving synthetic content, screenshots, or high-fidelity photography without any quality loss. [29, 30]
* Progressive Decoding: Images render smoothly from blurry to sharp as they download, which feels much faster to users.

## The Catch (Why you might not use it yet)

Despite its technical superiority, JPEG XL has a major browser compatibility issue. [26]

| Feature / Format    | WebP                | AVIF                  | JPEG XL (JXL)                          |
|---------------------|---------------------|-----------------------|----------------------------------------|
| Primary Champion    | Google              | Open-Source (AOMedia) | ISO / Joint Photographic Experts Group |
| Compression Quality | Good                | Superior (Tiny files) | Best for Lossless & Editing            |
| Browser Support     | ~97% (Universal)    | ~93% (Broad support)  | ~14% (Mostly limited to Safari)        |
| Current Web Status  | The "Safe" Baseline | The Production King   | The Future (Waiting on Chrome/Edge)    |

## Summary

If you want to use the absolute best format that works for almost everyone visiting your site today, use AVIF. If you want to experiment with the cutting edge of image technology—knowing it will mostly only render for Safari users right now—look into JPEG XL. [2, 3, 8]
Are you experimenting with this for a personal project, or looking to optimize a live production website?

---

If it is an impressive new format causing a stir in modern web development, you are almost certainly thinking of one of these cutting-edge visual technologies: [15]

## 1. glTF / GLB (The 3D Asset Format)

If the "impressive websites" you are thinking of let you interactively rotate, zoom, or animate flawless 3D objects right in your browser, this is it. [16, 17]

* What it is: glTF (Graphics Language Transmission Format) is known as the "JPEG of 3D". GLB is its binary file extension.
* Why it’s impressive: It streams complex 3D models, textures, and lighting data into a website efficiently. Combined with libraries like Three.js, it powers the highly immersive, cinematic portfolio and product pages trending across the web. [16, 18, 19]

## 2. Lottie / DotLottie (The Animation Format)

If the websites feature incredibly smooth, intricate, vector animations that react to your mouse or scrolling without lagging, you are thinking of Lottie.

* What it is: A JSON-based animation file format exported directly from Adobe After Effects.
* Why it’s impressive: It allows complex, interactive motion graphics to run at 60fps on web pages with tiny file sizes, entirely replacing heavy video files or clunky GIFs.

## 3. WebGPU (The Next-Gen Graphics API)

If you heard a technical name that sounded like "WebXL", you might have mixed up the letters of WebGPU. [20]

* What it is: The brand-new successor to WebGL.
* Why it’s impressive: It gives web browsers direct, high-performance access to your computer's graphics card hardware. It enables mind-blowing, console-quality 3D visuals, fluid simulations, and machine learning models to run directly on a web page without plugins. [17, 19, 20]

## 4. WebXR (The Spatial Format)

Another letter combination that fits the bill is WebXR. [19, 20]

* What it is: The unified web format for Virtual Reality (VR) and Augmented Reality (AR).
* Why it’s impressive: It allows immersive, spatial web design where you can project a 3D product directly into your physical room using your phone's camera, or view a site through a VR headset. [17, 19, 20]

## 5. Bento Grid (The Layout Format)

If you meant a visual layout structure rather than a file format, you are thinking of the Bento Grid. [21, 22]

* What it is: A layout style inspired by Apple's product promotional grids and Windows Phone tiles, relying heavily on CSS Grid technology. [23]
* Why it’s impressive: It organizes information into sleek, asymmetrical, rounded rectangular compartments that adapt beautifully to any screen size. [22]

Do any of these names—glTF/GLB, Lottie, WebGPU, WebXR, or Bento—ring the bell you were looking for?

## SOURCES

[1] [https://en.wikipedia.org](https://en.wikipedia.org/wiki/Non-Newtonian_fluid)
[2] [https://littlebinsforlittlehands.com](https://littlebinsforlittlehands.com/non-newtonian-fluids-cornstarch-science-activity/)
[3] [https://www.youtube.com](https://www.youtube.com/shorts/b66bdJ45X5E)
[4] [https://www.youtube.com](https://www.youtube.com/watch?v=qv1tpZ14xLU)
[5] [https://www.ebsco.com](https://www.ebsco.com/research-starters/science/non-newtonian-fluid)
[6] [https://www.sciencelearn.org.nz](https://www.sciencelearn.org.nz/resources/1502-non-newtonian-fluids)
[7] [https://kids.kiddle.co](https://kids.kiddle.co/Non-Newtonian_fluid)
[8] [https://www.reddit.com](https://www.reddit.com/r/explainlikeimfive/comments/16ukj5x/eli5_what_exactly_is_nonnewtonian_about/)
[9] [https://shop.nanografi.com](https://shop.nanografi.com/blog/nonnewtonian-fluids-and-their-place-in-the-everyday-life/)
[10] [https://www.youtube.com](https://www.youtube.com/watch?v=2IkHlFvBM20&t=15)
[11] [https://www.youtube.com](https://www.youtube.com/watch?v=Fnd-2jetT1w&t=152)
[12] [https://www.reddit.com](https://www.reddit.com/r/explainlikeimfive/comments/1pzvq1m/eli5_what_is_a_newtonian_fluid_and_how_is_it/)
[13] [https://www.reddit.com](https://www.reddit.com/r/UnearthedArcana/comments/razqbm/nonnewtonian_armor_a_revers_ac_armore_aka_my_most/)
[14] [https://www.facebook.com](https://www.facebook.com/gyaancore/videos/why-does-toothpaste-sit-like-a-solid-on-your-brush-but-flow-like-a-liquid-when-y/1507228610509608/)
[15] [https://cloudinary.com](https://cloudinary.com/blog/modern-image-formats-explained-choosing-best-web-use)
[16] [https://www.web3d.org](https://www.web3d.org/blog/anitahavele/webgl-gltf-and-x3d-three-pillars-interactive-3d-web)
[17] [https://javascript.plainenglish.io](https://javascript.plainenglish.io/building-3d-web-experiences-using-javascript-a-complete-guide-for-2026-b92c2880ecd3)
[18] [https://www.youtube.com](https://www.youtube.com/watch?v=aWFXHu9zXdQ&t=130)
[19] [https://www.youtube.com](https://www.youtube.com/watch?v=67tKmJULq0E&vl=en&t=2)
[20] [https://threejsresources.com](https://threejsresources.com/blog/the-future-of-the-3d-web-trends-for-2025-and-beyond)
[21] [https://cubitrek.com](https://cubitrek.com/blog/top-10-website-design-trends-for-2026-the-ultimate-guide)
[22] [https://www.bluecompass.com](https://www.bluecompass.com/blog/web-design-trends-to-watch-for)
[23] [https://cmscritic.com](https://cmscritic.com/wix-introduces-editor-x-offers-designers-advanced-tools-for-building-custom-websites)
[24] [https://customcodedwebsites.com](https://customcodedwebsites.com/blog/webp-vs-avif-vs-jpeg-xl-2025/)
[25] [https://hiredigital.com](https://hiredigital.com/blog/jpg-vs-webp-vs-avif-2026)
[26] [https://lessmb.com](https://lessmb.com/blog/avif-vs-webp-which-format-should-you-use-in-2026)
[27] [https://www.corewebvitals.io](https://www.corewebvitals.io/pagespeed/jpeg-xl-core-web-vitals-support)
[28] [https://www.reddit.com](https://www.reddit.com/r/jpegxl/comments/pdx4e8/jpeg_xl_developer_usage/)
[29] [https://www.reddit.com](https://www.reddit.com/r/AV1/comments/1ui1q7f/compressing_images_for_the_web_in_2026_who_wins/)
[30] [https://www.corewebvitals.io](https://www.corewebvitals.io/pagespeed/jpeg-xl-core-web-vitals-support)
[31] [https://lessflux.com](https://lessflux.com/avif-vs-webp-which-format-actually-speeds-up-your-site-in-2026/)

---
