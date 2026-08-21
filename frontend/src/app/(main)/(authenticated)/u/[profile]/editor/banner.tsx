import style from "./banner.module.scss";
import getIdColor from "@lib/getIdColor";
import React from "react";
import { PublicUserProfileResponse } from "@/api/user/profile";
import { Image } from "@/app/_components/ui/chromatic/Image";

interface BannerProps {
    userId: string;
    banner: string | null;
    banner_thumbhash: string | null;
    is_owner?: boolean; // optional prop to indicate if the user is the owner of the profile
    setProfile: React.Dispatch<
        React.SetStateAction<PublicUserProfileResponse | null>
    >; // optional function to update the profile state
}

function isBannerAnimated(banner: string): boolean {
    // prefix with "a_" indicates animated banner
    return banner.startsWith("a_");
}

export function Banner({ userId, banner, banner_thumbhash }: BannerProps) {
    const [bannerInitWidth, setBannerInitWidth] = React.useState(600);
    const [isinit, setIsInit] = React.useState(false);
    const [bannerInitHeight, setBannerInitHeight] = React.useState(240);

    const [bannerContainerWidth, setBannerContainerWidth] = React.useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] =
        React.useState(240);
    let bannerContainerRef = React.createRef<HTMLDivElement>();

    React.useEffect(() => {
        const handleResize = () => {
            if (bannerContainerRef.current) {
                setBannerContainerWidth(
                    Math.floor(bannerContainerRef.current.offsetWidth),
                );
                setBannerContainerHeight(
                    // set to int not float to avoid fractional pixels which can cause blurry images
                    Math.floor(
                        (bannerContainerRef.current.offsetWidth / 5) * 2,
                    ),
                ); // maintain aspect ratio 5 / 2
            }
        };

        if (bannerContainerRef.current && !isinit) {
            setBannerInitWidth(
                Math.floor(bannerContainerRef.current.offsetWidth),
            );
            setBannerInitHeight(
                Math.floor((bannerContainerRef.current.offsetWidth / 5) * 2),
            );
            setIsInit(true);
        }

        // Initial size
        handleResize();

        window.addEventListener("resize", handleResize);
        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [bannerContainerRef]);

    return (
        <div
            ref={bannerContainerRef}
            className={style["profile-banner-container"]}
        >
            {banner ? (
                <Image
                    className={style["profile-banner"]}
                    src={`banners/${userId}/${banner}`}
                    animated_src={
                        isBannerAnimated(banner)
                            ? `banners/${userId}/${banner.replace(".png", ".webp")}`
                            : undefined
                    }
                    optimizationType="animated_in_viewport"
                    alt="User Banner"
                    width={bannerInitWidth}
                    height={bannerInitHeight}
                    containerWidth={bannerContainerWidth}
                    containerHeight={bannerContainerHeight}
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
        </div>
    );
}
