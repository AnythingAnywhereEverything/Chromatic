import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogTrigger,
} from "@/app/_components/ui/chromatic/dialogue";
import { useState } from "react";
import React from "react";
import style from "../style.module.scss";
import { FaPen } from "react-icons/fa";
import { IoMdClose } from "react-icons/io";
import { BiSolidImageAdd } from "react-icons/bi";
import { AvatarCropper } from "./avatarCropper";
import { patchUserAvatar, PublicUserProfileResponse } from "@/api/user/profile";

export function EditAvatarPopup({
    onUpdate,
}: {
    onUpdate: (res: PublicUserProfileResponse) => void;
}) {
    const [isOpen, setIsOpen] = useState(false);
    const [cropOpen, setCropOpen] = useState(false);
    const [file, setFile] = useState<File | null>(null);

    const [cropPosX, setCropPosX] = useState(0.5);
    const [cropPosY, setCropPosY] = useState(0.5);
    const [cropScale, setCropScale] = useState(1.0);

    const [imageSize, setImageSize] = useState({
        width: 0,
        height: 0,
    });

    const handleFileUpload = () => {
        if (!file) return;

        // create form data to send to the server
        const formData = new FormData();
        formData.append("uploaded_avatar", file);
        formData.append("position_x", cropPosX.toString());
        formData.append("position_y", cropPosY.toString());
        formData.append("scale", cropScale.toString());

        // send the form data to the
        patchUserAvatar(formData).then((response) => {
            if (response) {
                console.log("Avatar updated successfully:", response);
                // Optionally, you can close the crop dialog after successful upload
                setCropOpen(false);
                onUpdate(response); // Call the onUpdate callback with the new avatar URL
            } else {
                // handle error
            }
        });
    };

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

    const clampCropPosition = (x: number, y: number) => {
        return {
            x: Math.min(1, Math.max(0, x)),
            y: Math.min(1, Math.max(0, y)),
        };
    };

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
                overlayClassName={style["edit-avatar-dialog-overlay"]}
            >
                <DialogTrigger asChild>
                    <button
                        className={style["edit-avatar-overlay"]}
                        onClick={() => setIsOpen(true)}
                    >
                        <div className={style["edit-avatar-button"]}>
                            <FaPen />
                        </div>
                    </button>
                </DialogTrigger>

                <DialogContent className={style["edit-avatar-dialog"]}>
                    <div className={style["edit-avatar-dialog-content"]}>
                        <div className={style["edit-avatar-dialog-header"]}>
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

                        <input
                            type="file"
                            accept="image/*"
                            className={style["file-input"]}
                            ref={fileInputRef}
                            onChange={handleFileChange}
                        />
                    </div>
                </DialogContent>
            </Dialog>

            {file && (
                <Dialog
                    open={cropOpen}
                    onOpenChange={setCropOpen}
                    overlayClassName={style["avatar-cropper-dialog-overlay"]}
                >
                    <DialogContent className={style["avatar-cropper-dialog"]}>
                        <div className={style["avatar-cropper-header"]}>
                            <h2>Edit Image</h2>

                            <DialogClose
                                className={style["close-button"]}
                                aria-label="Close"
                                onClick={() => setCropOpen(false)}
                            >
                                <IoMdClose />
                            </DialogClose>
                        </div>

                        <div className={style["avatar-cropper-content"]}>
                            {file && imageSize.width > 0 && (
                                <AvatarCropper
                                    file={file}
                                    cropPosX={cropPosX}
                                    cropPosY={cropPosY}
                                    cropScale={cropScale}
                                    onCropPositionChange={updateCropPosition}
                                    onCropScaleChange={updateCropScale}
                                />
                            )}
                        </div>
                        <div className={style["avatar-cropper-footer"]}>
                            <button
                                className={style["cancel-button"]}
                                onClick={() => setCropOpen(false)}
                            >
                                Cancel
                            </button>
                            <button
                                className={style["save-button"]}
                                onClick={() => {
                                    // print everything to console for now
                                    console.log(
                                        "Saving avatar with crop position:",
                                        cropPosX,
                                        cropPosY,
                                        "and scale:",
                                        cropScale,
                                    );
                                    handleFileUpload();
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
