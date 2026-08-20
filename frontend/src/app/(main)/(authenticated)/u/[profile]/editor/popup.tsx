import { PublicUserProfileResponse } from "@/api/user/profile";
import { useUserService } from "@/hooks/useUserService";
import React from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogTrigger,
} from "@/app/_components/ui/chromatic/dialogue";
import style from "./popup.module.scss";
import { IoMdClose } from "react-icons/io";
import { BiSolidImageAdd } from "react-icons/bi";

export interface imageUploadProps {
    File: File;
    PositionX: number;
    PositionY: number;
    Scale: number;
}

interface ImageEditorProps {
    headerText?: string;
    descriptionText?: string;
    ratio?: [number, number];
    cropSelectorStyle?: string;
    triggerElement?: React.ReactNode;
    onUpload: (data: imageUploadProps) => Promise<void>;
}

interface ImageCropperProps {
    file: File;
    cropPosX: number;
    cropPosY: number;
    cropScale: number;
    cropSelectorStyle?: string;
    ratio: [number, number];
    onCropPositionChange: (x: number, y: number) => void;
    onCropScaleChange: (scale: number) => void;
}

const clampCropPosition = (x: number, y: number) => {
    return {
        x: Math.min(1, Math.max(0, x)),
        y: Math.min(1, Math.max(0, y)),
    };
};

function ImageCropper({
    file,
    cropPosX,
    cropPosY,
    ratio,
    cropScale,
    onCropPositionChange,
    onCropScaleChange,
    cropSelectorStyle = style["cropper-selection"],
}: ImageCropperProps) {
    const [imageUrl, setImageUrl] = React.useState<string | null>(null);

    const [imageSize, setImageSize] = React.useState({
        width: 0,
        height: 0,
    });

    const containerRef = React.useRef<HTMLDivElement>(null);

    const dragRef = React.useRef({
        dragging: false,
        startX: 0,
        startY: 0,
        startCropX: 0.5,
        startCropY: 0.5,
    });

    React.useEffect(() => {
        const url = URL.createObjectURL(file);

        setImageUrl(url);

        const image = new Image();

        image.onload = () => {
            setImageSize({
                width: image.naturalWidth,
                height: image.naturalHeight,
            });
        };

        image.src = url;

        return () => {
            URL.revokeObjectURL(url);
        };
    }, [file]);

    const getGeometry = () => {
        const container = containerRef.current;

        if (!container || !imageSize.width || !imageSize.height) {
            return null;
        }

        const previewWidth = container.clientWidth;
        const previewHeight = container.clientHeight;

        const ratioWidth = ratio[0];
        const ratioHeight = ratio[1];

        // * Start with a crop area that is 90% of the preview height.
        let cropHeight = previewHeight * 0.9;
        let cropWidth = cropHeight * (ratioWidth / ratioHeight);

        // * Keep wide ratios inside the preview.
        const maxCropWidth = previewWidth * 0.9;

        if (cropWidth > maxCropWidth) {
            cropWidth = maxCropWidth;

            cropHeight = cropWidth * (ratioHeight / ratioWidth);
        }

        // * Scale the image so the crop area is completely covered.
        const fitScale = Math.max(
            cropWidth / imageSize.width,
            cropHeight / imageSize.height,
        );

        const fittedWidth = imageSize.width * fitScale;

        const fittedHeight = imageSize.height * fitScale;

        // * Lower cropScale means a smaller crop area and therefore more zoom.
        const scaledWidth = fittedWidth / cropScale;

        const scaledHeight = fittedHeight / cropScale;

        return {
            previewWidth,
            previewHeight,
            cropWidth,
            cropHeight,
            fittedWidth,
            fittedHeight,
            scaledWidth,
            scaledHeight,
        };
    };

    const getImagePosition = () => {
        const geometry = getGeometry();

        if (!geometry) {
            return {
                left: "50%",
                top: "50%",
            };
        }

        const movementWidth = Math.max(
            0,
            geometry.scaledWidth - geometry.cropWidth,
        );

        const movementHeight = Math.max(
            0,
            geometry.scaledHeight - geometry.cropHeight,
        );

        const imageOffsetX = (0.5 - cropPosX) * movementWidth;

        const imageOffsetY = (0.5 - cropPosY) * movementHeight;

        return {
            left: `calc(50% + ${imageOffsetX}px)`,
            top: `calc(50% + ${imageOffsetY}px)`,
        };
    };

    const handlePointerDown = (event: React.PointerEvent<HTMLDivElement>) => {
        event.currentTarget.setPointerCapture(event.pointerId);

        dragRef.current = {
            dragging: true,
            startX: event.clientX,
            startY: event.clientY,
            startCropX: cropPosX,
            startCropY: cropPosY,
        };
    };

    const handlePointerMove = (event: React.PointerEvent<HTMLDivElement>) => {
        if (!dragRef.current.dragging) {
            return;
        }

        const geometry = getGeometry();

        if (!geometry) {
            return;
        }

        const movementWidth = Math.max(
            0,
            geometry.scaledWidth - geometry.cropWidth,
        );
        const movementHeight = Math.max(
            0,
            geometry.scaledHeight - geometry.cropHeight,
        );

        const deltaX = event.clientX - dragRef.current.startX;
        const deltaY = event.clientY - dragRef.current.startY;

        const nextX =
            movementWidth > 0
                ? dragRef.current.startCropX - deltaX / movementWidth
                : dragRef.current.startCropX;

        const nextY =
            movementHeight > 0
                ? dragRef.current.startCropY - deltaY / movementHeight
                : dragRef.current.startCropY;

        onCropPositionChange(nextX, nextY);
    };

    const handlePointerUp = (event: React.PointerEvent<HTMLDivElement>) => {
        dragRef.current.dragging = false;

        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
            event.currentTarget.releasePointerCapture(event.pointerId);
        }
    };

    const handleWheel = (event: React.WheelEvent<HTMLDivElement>) => {
        const zoomSpeed = 0.0005;

        const nextScale = cropScale + event.deltaY * zoomSpeed;

        onCropScaleChange(Math.min(1, Math.max(0.1, nextScale)));
    };

    const position = getImagePosition();

    return (
        <div className={style["cropper-wrapper"]}>
            <div
                ref={containerRef}
                className={style["cropper"]}
                onPointerDown={handlePointerDown}
                onPointerMove={handlePointerMove}
                onPointerUp={handlePointerUp}
                onPointerCancel={handlePointerUp}
                onWheel={handleWheel}
            >
                {imageUrl && (
                    <img
                        src={imageUrl}
                        className={style["cropper-image"]}
                        draggable={false}
                        alt=""
                        style={{
                            width: getGeometry()
                                ? `${getGeometry()!.scaledWidth}px`
                                : undefined,

                            height: getGeometry()
                                ? `${getGeometry()!.scaledHeight}px`
                                : undefined,

                            left: position.left,
                            top: position.top,
                        }}
                    />
                )}

                <div className={cropSelectorStyle} />
            </div>

            <div className={style["cropper-controls"]}>
                <input
                    type="range"
                    min="0.1"
                    max="1"
                    step="0.01"
                    value={cropScale}
                    onChange={(event) => {
                        onCropScaleChange(Number(event.target.value));
                    }}
                    style={{
                        direction: "rtl",
                    }}
                />
            </div>
        </div>
    );
}

export function ImageEditor({
    ratio = [1, 1],
    triggerElement,
    onUpload,
    cropSelectorStyle = style["cropper-selection"],
}: ImageEditorProps) {
    const [isOpen, setIsOpen] = React.useState(false);
    const [cropOpen, setCropOpen] = React.useState(false);
    const [file, setFile] = React.useState<File | null>(null);

    const [cropPosX, setCropPosX] = React.useState(0.5);
    const [cropPosY, setCropPosY] = React.useState(0.5);
    const [cropScale, setCropScale] = React.useState(1.0);

    const [imageSize, setImageSize] = React.useState({
        width: 0,
        height: 0,
    });

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const selectedFile = event.target.files?.[0] || null;

        if (!selectedFile) {
            return;
        }

        const imageUrl = URL.createObjectURL(selectedFile);

        const image = new Image();

        image.onload = () => {
            setImageSize({
                width: image.naturalWidth,
                height: image.naturalHeight,
            });

            console.log(
                "Loaded image dimensions:",
                image.naturalWidth,
                image.naturalHeight,
            );

            // * Every newly selected image starts centered.
            setCropPosX(0.5);
            setCropPosY(0.5);
            setCropScale(1.0);

            URL.revokeObjectURL(imageUrl);
        };

        image.src = imageUrl;

        setFile(selectedFile);
        setIsOpen(false);
        setCropOpen(true);
    };

    const fileInputRef = React.useRef<HTMLInputElement>(null);

    const updateCropPosition = (x: number, y: number) => {
        const clamped = clampCropPosition(x, y);

        setCropPosX(clamped.x);
        setCropPosY(clamped.y);
    };

    const updateCropScale = (scale: number) => {
        const nextScale = Math.min(1, Math.max(0.1, scale));

        setCropScale(nextScale);

        const clamped = clampCropPosition(cropPosX, cropPosY);

        setCropPosX(clamped.x);
        setCropPosY(clamped.y);
    };

    return (
        <>
            <Dialog
                open={isOpen}
                onOpenChange={setIsOpen}
                overlayClassName={style["overlay"]}
            >
                <DialogTrigger asChild onClick={() => setIsOpen(true)}>{triggerElement}</DialogTrigger>

                <DialogContent className={style["content"]}>
                    <div className={style["editor-header"]}>
                        <div>
                            <h2>Edit Avatar</h2>
                            <p>Choose a new avatar image to upload.</p>
                        </div>

                        <DialogClose
                            className={style["close-button"]}
                            aria-label="Close"
                        >
                            <IoMdClose />
                        </DialogClose>
                    </div>

                    <button
                        name="upload-button"
                        className={style["upload-button"]}
                        onClick={() => {
                            fileInputRef.current?.click();
                        }}
                    >
                        <BiSolidImageAdd />
                        <p>Upload Image</p>
                    </button>

                    <p className={style["upload-instructions"]}>
                        Max image size 10MB
                    </p>

                    <input
                        type="file"
                        accept="image/*"
                        className={style["file-input"]}
                        ref={fileInputRef}
                        onChange={handleFileChange}
                    />
                </DialogContent>
            </Dialog>

            {file && (
                <Dialog
                    open={cropOpen}
                    onOpenChange={setCropOpen}
                    overlayClassName={style["overlay"]}
                >
                    <DialogContent className={style["content"]}>
                        <div className={style["cropper-header"]}>
                            <h2>Edit Image</h2>

                            <DialogClose
                                className={style["close-button"]}
                                aria-label="Close"
                                onClick={() => setCropOpen(false)}
                            >
                                <IoMdClose />
                            </DialogClose>
                        </div>

                        <div className={style["cropper-content"]}>
                            {file && imageSize.width > 0 && (
                                <ImageCropper
                                    file={file}
                                    cropPosX={cropPosX}
                                    cropPosY={cropPosY}
                                    cropScale={cropScale}
                                    ratio={ratio}
                                    onCropPositionChange={updateCropPosition}
                                    onCropScaleChange={updateCropScale}
                                    cropSelectorStyle={cropSelectorStyle}
                                />
                            )}
                        </div>
                        <div className={style["cropper-footer"]}>
                            <button
                                className={style["cancel-button"]}
                                onClick={() => setCropOpen(false)}
                            >
                                Cancel
                            </button>
                            <button
                                className={style["save-button"]}
                                onClick={async () => {
                                    await onUpload({
                                        File: file,
                                        PositionX: cropPosX,
                                        PositionY: cropPosY,
                                        Scale: cropScale,
                                    });
                                    setCropOpen(false);
                                }}
                            >
                                Save
                            </button>
                        </div>
                    </DialogContent>
                </Dialog>
            )}
        </>
    );
}
