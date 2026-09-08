import { cache } from "react";
import { getPublicUserProfile } from "@/api/user/profile";
import ProfileBody from "./body";

const getProfile = cache(async (profile: string) => {
    return getPublicUserProfile(profile);
});

export const generateMetadata = async ({
    params,
}: {
    params: Promise<{ profile: string }>;
}) => {
    const { profile: profileOf } = await params;

    const response = await getProfile(profileOf);

    if (!response) {
        return {
            title: "User not found",
        };
    }

    const images = [];

    if (response.avatar) {
        images.push({
            url: `${process.env.NEXT_PUBLIC_CDN_URL}avatars/${response.id}/${response.avatar}`,
            width: 400,
            height: 400,
        });
    }

    if (response.banner) {
        images.push({
            url: `${process.env.NEXT_PUBLIC_CDN_URL}banners/${response.id}/${response.banner}`,
            width: 1200,
            height: 400,
        });
    }

    return {
        title: `${profileOf}'s Profile`,
        description: `${response.display_name} (@${response.username}) - ${response.bio || "No bio"}`,
        openGraph: {
            title: `${profileOf}'s Profile`,
            description: `${response.display_name} (@${response.username}) - ${response.bio || "No bio"}`,
            url: `${process.env.NEXT_PUBLIC_URL}${profileOf}`,
            siteName: "Chromatic",
            images,
            locale: "en-US",
            type: "website",
        },
    };
};

export default async function ProfilePage({
    params,
}: {
    params: Promise<{ profile: string }>;
}) {
    const { profile } = await params;

    const response = await getProfile(profile);

    if (!response) {
        return null;
    }

    return <ProfileBody profile={response} />;
}