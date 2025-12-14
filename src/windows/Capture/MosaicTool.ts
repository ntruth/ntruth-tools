import Konva from 'konva'

export interface MosaicOptions {
  pixelSize: number
}

export interface MosaicRect {
  x: number
  y: number
  width: number
  height: number
}

/**
 * Creates a pixelated (mosaic) overlay node.
 *
 * Strategy:
 * - Use the original screenshot image as the source
 * - Crop to the requested rect
 * - Apply Konva.Filters.Pixelate
 * - Cache the node so the filter is GPU-accelerated / fast
 */
export function createMosaicNode(
  sourceImage: HTMLImageElement,
  rect: MosaicRect,
  options: MosaicOptions,
): Konva.Image {
  const image = new Konva.Image({
    x: rect.x,
    y: rect.y,
    width: rect.width,
    height: rect.height,
    image: sourceImage,
    crop: {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    },
    listening: true,
    draggable: true,
  })

  // Pixelate filter requires caching.
  image.filters([Konva.Filters.Pixelate])
  image.pixelSize(Math.max(2, Math.round(options.pixelSize)))
  image.cache()

  return image
}
