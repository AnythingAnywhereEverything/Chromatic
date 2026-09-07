"use client";

const Vips = require("wasm-vips");

let vipsPromise: Promise<any> | null = null;

function getVips() {
    if (!vipsPromise) {
        vipsPromise = Vips({
            mainScriptUrlOrBlob: "/wasm-vips/vips.js",
            locateFile: (path: string) => {
                if (path.endsWith("vips.wasm")) {
                    return "/wasm-vips/vips.wasm";
                }

                if (path.endsWith("vips-heif.wasm")) {
                    return "/wasm-vips/vips-heif.wasm";
                }

                if (path.endsWith("vips-jxl.wasm")) {
                    return "/wasm-vips/vips-jxl.wasm";
                }

                if (path.endsWith("vips-resvg.wasm")) {
                    return "/wasm-vips/vips-resvg.wasm";
                }

                return path;
            },
        });
    }

    return vipsPromise;
}

export interface ProcessResult {
    final: Blob;
    static?: Blob;
    filetype: string;
}

export type CropStyle =
    | {
          type: "absolute";
          width: number;
          height: number;
      }
    | {
          type: "normalized";
          width: number;
          height: number;
      }
    | {
          type: "ratio";
          width: number;
          height: number;
          scale: number;
      };

export interface CropPosition {
    x: number;
    y: number;
}

export interface PreCalculatedCrop {
    left: number;
    top: number;
    crop_width: number;
    crop_height: number;
}

function computeNormalize(
    width: number,
    height: number,
    imageWidth: number,
    imageHeight: number,
): [number, number] {
    return [Math.round(width * imageWidth), Math.round(height * imageHeight)];
}

function computeRatio(
    ratioWidth: number,
    ratioHeight: number,
    scale: number,
    imageWidth: number,
    imageHeight: number,
): [number, number] {
    if (scale <= 0) {
        throw new Error(`Invalid crop scale: ${scale}`);
    }

    // * Maximum crop dimensions that fit while maintaining the aspect ratio.
    const aspect = ratioWidth / ratioHeight;

    let maxCropWidth: number;
    let maxCropHeight: number;

    if (imageWidth / imageHeight >= aspect) {
        maxCropHeight = imageHeight;
        maxCropWidth = Math.round(maxCropHeight * aspect);
    } else {
        maxCropWidth = imageWidth;
        maxCropHeight = Math.round(maxCropWidth / aspect);
    }

    return [
        Math.round(maxCropWidth * scale),
        Math.round(maxCropHeight * scale),
    ];
}

export function computeCropDimensions(
    imageWidth: number,
    imageHeight: number,
    style: CropStyle,
    position?: CropPosition,
): PreCalculatedCrop {
    let cropWidth: number;
    let cropHeight: number;

    switch (style.type) {
        case "absolute":
            cropWidth = style.width;
            cropHeight = style.height;
            break;

        case "normalized":
            [cropWidth, cropHeight] = computeNormalize(
                style.width,
                style.height,
                imageWidth,
                imageHeight,
            );
            break;

        case "ratio":
            [cropWidth, cropHeight] = computeRatio(
                style.width,
                style.height,
                style.scale,
                imageWidth,
                imageHeight,
            );
            break;
    }

    // * Keep the crop inside the source image.
    cropWidth = Math.min(cropWidth, imageWidth);
    cropHeight = Math.min(cropHeight, imageHeight);

    const maxLeft = Math.max(imageWidth - cropWidth, 0);
    const maxTop = Math.max(imageHeight - cropHeight, 0);

    let left: number;
    let top: number;

    if (position) {
        // * Position is normalized over the available movement range.
        left = Math.round(Math.min(Math.max(position.x, 0), 1) * maxLeft);

        top = Math.round(Math.min(Math.max(position.y, 0), 1) * maxTop);
    } else {
        left = Math.floor(maxLeft / 2);
        top = Math.floor(maxTop / 2);
    }

    return {
        left,
        top,
        crop_width: cropWidth,
        crop_height: cropHeight,
    };
}

export class ImageProcessor {
    private vips: Awaited<ReturnType<typeof Vips>>;
    private calculatedCrop?: PreCalculatedCrop;

    private constructor(vips: Awaited<ReturnType<typeof Vips>>) {
        this.vips = vips;
    }

    static async create(): Promise<ImageProcessor> {
        if (typeof window === "undefined") {
            throw new Error("ImageProcessor must run in the browser");
        }

        return new ImageProcessor(await getVips());
    }

    private processCrop(
        image: any,
        style: CropStyle,
        position?: CropPosition,
    ): any {
        const crop =
            this.calculatedCrop ??
            (this.calculatedCrop = computeCropDimensions(
                image.width,
                image.height,
                style,
                position,
            ));

        return image.crop(
            crop.left,
            crop.top,
            crop.crop_width,
            crop.crop_height,
        );
    }

    private processStatic(
        image: any,
        cropStyle: CropStyle,
        position?: CropPosition,
    ): any {
        return this.processCrop(image, cropStyle, position);
    }

    private processAnimated(
        image: any,
        cropStyle: CropStyle,
        position?: CropPosition,
    ): any {
        const nPages = image.getInt("n-pages");
        const pageHeight = image.getInt("page-height");

        const crop =
            this.calculatedCrop ??
            (this.calculatedCrop = computeCropDimensions(
                image.width,
                pageHeight,
                cropStyle,
                position,
            ));

        const frames: any[] = [];

        try {
            for (let page = 0; page < nPages; page++) {
                const frame = image.crop(
                    crop.left,
                    page * pageHeight + crop.top,
                    crop.crop_width,
                    crop.crop_height,
                );

                frames.push(frame);
            }

            const result = this.vips.Image.arrayjoin(frames, {
                across: 1,
                hspacing: crop.crop_width,
                vspacing: crop.crop_height,
            });

            result.setInt("n-pages", nPages);
            result.setInt("page-height", crop.crop_height);

            return result;
        } finally {
            for (const frame of frames) {
                frame.delete();
            }
        }
    }

    async transform(
        file: File,
        cropStyle: CropStyle,
        position?: CropPosition,
    ): Promise<ProcessResult> {
        const input = new Uint8Array(await file.arrayBuffer());

        let image: any;
        let result: any;
        let staticThumbnail: any;

        try {
            const mime = file.type.toLowerCase();

            image =
                mime === "image/gif" || mime === "image/webp"
                    ? this.vips.Image.newFromBuffer(input, "n=-1")
                    : this.vips.Image.newFromBuffer(input);

            const isAnimated =
                mime === "image/gif" || mime === "image/webp"
                    ? (() => {
                          try {
                              return image.getInt("n-pages") > 1;
                          } catch {
                              // * Missing n-pages means the image was loaded as a single page.
                              return false;
                          }
                      })()
                    : false;

            result = isAnimated
                ? this.processAnimated(image, cropStyle, position)
                : this.processStatic(image, cropStyle, position);

            const output = result.writeToBuffer(".webp");

            const buffer = new ArrayBuffer(output.byteLength);
            new Uint8Array(buffer).set(output);

            staticThumbnail = this.vips.Image.thumbnailBuffer(buffer, 256, {
                size: "force",
            });

            const staticOutput = staticThumbnail.writeToBuffer(".webp");

            return {
                final: new Blob([buffer], {
                    type: "image/webp",
                }),

                static: new Blob([staticOutput], {
                    type: "image/webp",
                }),

                filetype: "image/webp",
            };
        } finally {
            // * Release libvips native/WASM objects after every transformation.
            staticThumbnail?.delete();
            result?.delete();
            image?.delete();
        }
    }
}
