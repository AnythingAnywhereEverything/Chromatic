import React, { useEffect, useState } from "react";
import style from "../style.module.scss";

export type AvatarCropperProps = {
    file: File;
    cropPosX: number;
    cropPosY: number;
    cropScale: number;
    onCropPositionChange: (x: number, y: number) => void;
    onCropScaleChange: (scale: number) => void;
};

export function AvatarCropper({
    file,
    cropPosX,
    cropPosY,
    cropScale,
    onCropPositionChange,
    onCropScaleChange,
}: AvatarCropperProps) {
    const [imageUrl, setImageUrl] = useState<string | null>(null);

    const [imageSize, setImageSize] = useState({
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

    useEffect(() => {
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

        // * Smaller crop area, while keeping the preview wide.
        const cropSize = previewHeight * 0.9;

        const fitScale = Math.max(
            cropSize / imageSize.width,
            cropSize / imageSize.height,
        );

        const fittedWidth = imageSize.width * fitScale;

        const fittedHeight = imageSize.height * fitScale;

        const scaledWidth = fittedWidth / cropScale;

        const scaledHeight = fittedHeight / cropScale;

        return {
            previewWidth,
            previewHeight,
            cropSize,
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
            geometry.scaledWidth - geometry.cropSize,
        );

        const movementHeight = Math.max(
            0,
            geometry.scaledHeight - geometry.cropSize,
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
            geometry.scaledWidth - geometry.cropSize,
        );

        const movementHeight = Math.max(
            0,
            geometry.scaledHeight - geometry.cropSize,
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
        const zoomSpeed = 0.0015;

        const nextScale = cropScale - event.deltaY * zoomSpeed;

        onCropScaleChange(Math.min(1, Math.max(0.1, nextScale)));
    };

    const position = getImagePosition();

    return (
        <div className={style["avatar-cropper-wrapper"]}>
            <div
                ref={containerRef}
                className={style["avatar-cropper"]}
                onPointerDown={handlePointerDown}
                onPointerMove={handlePointerMove}
                onPointerUp={handlePointerUp}
                onPointerCancel={handlePointerUp}
                onWheel={handleWheel}
            >
                {imageUrl && (
                    <img
                        src={imageUrl}
                        className={style["avatar-cropper-image"]}
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

                <div className={style["avatar-cropper-selection"]} />
            </div>

            <div className={style["avatar-cropper-controls"]}>
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
