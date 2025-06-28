# GitHub Repository Social Media Preview Setup

The repository includes a custom social media preview banner at `assets/github-banner.svg`.

## How to Set Up the Banner on GitHub

1. Go to your repository settings on GitHub
2. Scroll down to the "Social Preview" section
3. Click "Edit" and then "Upload an image"
4. Upload the `assets/github-banner.svg` file (or convert it to PNG first if needed)

## Banner Specifications

- **Dimensions**: 1280×640 pixels (optimal for social media)
- **Format**: SVG (scalable vector graphics)
- **Design Elements**:
  - Gradient background (blue to purple)
  - CPU chip visualization with MIPS label
  - Project title and subtitle
  - Feature highlights
  - Version badge (v0.2.0)
  - Rust language badge

## Converting SVG to PNG (if needed)

If GitHub doesn't accept the SVG directly, you can convert it to PNG using:

```bash
# Using ImageMagick
convert -density 300 assets/github-banner.svg assets/github-banner.png

# Using Inkscape
inkscape assets/github-banner.svg --export-png=assets/github-banner.png --export-width=1280 --export-height=640
```

## Updating the Banner

To update the version number or other details in the banner, edit the `assets/github-banner.svg` file. The version badge is located around line 95 of the SVG code.
