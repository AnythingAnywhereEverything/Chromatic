import { getPublicUserProfile } from "@/api/user/profile";
import { fetchWithAuth, fetchWithOptionAuth } from "@/handler/token_handler";
import ProfileBanner from "./profileBanner";
import ProfileBody from "./body";

export const generateMetadata = async ({ params }: { params: Promise<{ profile: string }> }) => {
    const resolvedParams = await params;
    const profileOf = resolvedParams.profile;

    let response = await getPublicUserProfile(profileOf);

    if (!response) {
        return {
            title: "User not found",
        }
    }

    let images = [];

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
            images: images,
            locale: "en-US",
            type: "website",
        },
    }
}



export default async function ProfilePage({
  params,
}: {
  params: Promise<{ profile: string }>;
}) {

    return (
        <ProfileBody params={await params} />
    );
}