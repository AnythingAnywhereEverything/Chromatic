import { ChromaImage } from "@/app/_components/ui/chromatic/chromaImage";
import style from "./banner.module.scss";
import getIdColor from "@lib/getIdColor";
import { FaPen } from "react-icons/fa";
import React from "react";
import { ImageEditor } from "./popup";
import { useUserService } from "@/hooks/useUserService";
import { PublicUserProfileResponse } from "@/api/user/profile";

interface BannerProps {
    userId: string;
    banner: string | null;
    banner_thumbhash: string | null;
    is_owner?: boolean; // optional prop to indicate if the user is the owner of the profile
    setProfile: React.Dispatch<React.SetStateAction<PublicUserProfileResponse | null>>; // optional function to update the profile state
}

function isBannerAnimated(banner: string): boolean {
    // prefix with "a_" indicates animated banner
    return banner.startsWith("a_");
}

export function Banner({
    userId,
    banner,
    banner_thumbhash,
    is_owner = false,
    setProfile,
}: BannerProps) {
    const [bannerSrc, setBannerSrc] = React.useState<string | null>(null);

    const [bannerContainerWidth, setBannerContainerWidth] = React.useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] = React.useState(240);
    let bannerContainerRef = React.createRef<HTMLDivElement>();

    React.useEffect(() => {
        const handleResize = () => {
            console.log(
                "Banner container size:",
                bannerContainerWidth,
                bannerContainerHeight,
            );
            if (bannerContainerRef.current) {
                setBannerContainerWidth(bannerContainerRef.current.offsetWidth);
                setBannerContainerHeight(
                    // set to int not float to avoid fractional pixels which can cause blurry images
                    Math.floor((bannerContainerRef.current.offsetWidth / 5) * 2),
                ); // maintain aspect ratio 5 / 2
            }
        };

        // Initial size
        handleResize();

        window.addEventListener("resize", handleResize);
        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [bannerContainerRef]);

    React.useEffect(() => {
        if (banner) {
            setBannerSrc(`banners/${userId}/${banner}`);
        } else {
            setBannerSrc(null);
        }
    }, [userId, banner]);

    const userService = useUserService();

    const handleUpload = async (data: {
        File: File;
        PositionX: number;
        PositionY: number;
        Scale: number;
    }) => {
        const formData = new FormData();
        formData.append("uploaded_banner", data.File);
        formData.append("position_x", data.PositionX.toString());
        formData.append("position_y", data.PositionY.toString());
        formData.append("scale", data.Scale.toString());

        await userService.updateUserBanner.mutateAsync(formData).then((res) => {
            if (res) {
                setBannerSrc(`banners/${res.id}/${res.banner}`);
                setProfile((prevProfile) => {
                    if (prevProfile) {
                        return {
                            ...prevProfile,
                            banner: res.banner,
                            banner_thumbhash: res.banner_thumbhash,
                        };
                    }
                    return prevProfile;
                });
            }
        });
    };

    return (
        <div 
            ref={bannerContainerRef}
            className={style["profile-banner-container"]}
            onMouseEnter={() => {
                if (banner && isBannerAnimated(banner)) {
                    console.log("Banner is animated, changing src to .webp");
                    // trim .png and replace with .webp for animated banner
                    const animatedBannerSrc = `banners/${userId}/${banner.replace(
                        ".png",
                        ".webp",
                    )}`;
                    setBannerSrc(animatedBannerSrc);
                }
            }}
            onMouseLeave={() => {
                setBannerSrc(
                    banner ? `banners/${userId}/${banner}` : null,
                );
            }}
        >
            {banner ? (
                <ChromaImage
                    className={style["profile-banner"]}
                    src={bannerSrc || ""}
                    alt="User Banner"
                    width={bannerContainerWidth}
                    height={bannerContainerHeight}
                    thumbhash={banner_thumbhash || undefined}
                />
            ) : (
                <div
                    className={style["default-banner"]}
                    style={{
                        backgroundColor: getIdColor(userId),
                        width: `${bannerContainerWidth}px`,
                        height: `${bannerContainerHeight}px`,
                    }}
                />
            )}

            {is_owner && (
                <ImageEditor
                    cropSelectorStyle={style["cropper-selection"]}
                    ratio={[5, 2]}
                    triggerElement={
                        <button className={style["edit-button-overlay"]}>
                            <div className={style["edit-button"]}>
                                <FaPen />
                                Edit Banner
                            </div>
                        </button>
                    }
                    onUpload={async (data) => {
                        await handleUpload(data);
                    }}
                />
            )}
        </div>
    );
}
