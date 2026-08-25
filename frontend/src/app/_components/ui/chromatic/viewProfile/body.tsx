import { IoMdClose } from "react-icons/io";
import { DialogClose } from "../dialogue";
import style from "./viewProfile.module.scss";
import { PublicUserProfileResponse } from "@/api/user/profile";
import type { AvatarPayload } from "./avatar";
import Banner, { BannerPayload } from "./banner";
import Avatar from "./avatar";
import BannerBackdrop from "./banner/backdrop";
import React, { useEffect } from "react";
import DisplayName from "./displayname";
import { Post } from "../post";
import { mediaPostProps } from "@/api/post/getFeed";
import EditableTextArea from "../textarea";

export type ProfilePayload = {
  display_name?: string;
  bio?: string;
  uploaded_avatar?: File;
  remove_avatar?: boolean;
  uploaded_banner?: File;
  remove_banner?: boolean;
  quote?: string;

  // non payload fields for internal use
  banner_preview_blob_url?: string | null;
};

interface ViewProfileBodyProps {
  profile: PublicUserProfileResponse;
  is_owner: boolean;
  // overrides the internal payload state
  payload?: ProfilePayload;
  onChange?: (payload: ProfilePayload) => void;
  recentPost?: mediaPostProps[];
}

interface AvatarData {
  avatarUrl: string | null;
  blobUrl: string | null;
  thumbhash: string | null;
}

interface BannerData {
  bannerUrl: string | null;
  blobUrl: string | null;
  previewBlobUrl?: string | null;
  thumbhash: string | null;
}

const ViewProfileBody = ({
  profile,
  is_owner: initialIsOwner,
  payload: externalPayload,
  onChange,
  recentPost = [],
}: ViewProfileBodyProps) => {
  const rootRef = React.useRef<HTMLDivElement>(null);
  const handleAvatarChange = (aPayload: AvatarPayload) => {
    onChange?.({
      ...externalPayload,
      uploaded_avatar: aPayload.file,
      remove_avatar: aPayload.remove,
    });
  };

  const handleBannerChange = (bPayload: BannerPayload) => {
    onChange?.({
      ...externalPayload,
      banner_preview_blob_url: bPayload.staticPreviewBlobUrl,
      uploaded_banner: bPayload.file,
      remove_banner: bPayload.remove,
    });
  };

  const handleDisplayNameChange = (newDisplayName: string) => {
    if (
      (profile.display_name === null && newDisplayName === "") ||
      profile.display_name === newDisplayName ||
      (profile.display_name === null && newDisplayName === profile.username)
    ) {
      onChange?.({
        ...externalPayload,
        display_name: undefined,
      });
    } else {
      onChange?.({
        ...externalPayload,
        display_name: newDisplayName,
      });
    }
  };

  const handleQuoteChange = (newQuote: string) => {
    if (
      (profile.quote === null && newQuote === "") ||
      profile.quote === newQuote ||
      (profile.quote === null && newQuote === profile.quote)
    ) {
      onChange?.({
        ...externalPayload,
        quote: undefined,
      });
    } else {
      onChange?.({
        ...externalPayload,
        quote: newQuote,
      });
    }
  };

  const handleBioChange = (newBio: string) => {
    if (
      (profile.bio === null && newBio === "") ||
      profile.bio === newBio ||
      (profile.bio === null && newBio === profile.bio)
    ) {
      onChange?.({
        ...externalPayload,
        bio: undefined,
      });
    } else {
      onChange?.({
        ...externalPayload,
        bio: newBio,
      });
    }
  };

  const [displayName, setDisplayName] = React.useState<string>(
    profile.display_name ? profile.display_name : profile.username,
  );

  const [avatarData, setAvatarData] = React.useState<AvatarData>({
    avatarUrl: profile.avatar,
    blobUrl: null,
    thumbhash: profile.avatar_thumbhash,
  });

  const [bannerData, setBannerData] = React.useState<BannerData>({
    bannerUrl: profile.banner,
    blobUrl: null,
    previewBlobUrl: null,
    thumbhash: profile.banner_thumbhash,
  });

  const [quote, setQuote] = React.useState<string>(
    profile.quote ? profile.quote : "",
  );

  const [bio, setBio] = React.useState<string>(
    profile.bio ? profile.bio : ""
  );

  //  * temporary

  React.useEffect(() => {
    if (externalPayload?.uploaded_avatar) {
      const blobUrl = URL.createObjectURL(externalPayload.uploaded_avatar);
      setAvatarData((prev) => ({
        ...prev,
        blobUrl: blobUrl,
        avatarUrl: null,
        thumbhash: null,
      }));
    } else if (externalPayload?.remove_avatar) {
      setAvatarData((prev) => ({
        ...prev,
        blobUrl: null,
        avatarUrl: null,
        thumbhash: null,
      }));
    } else {
      setAvatarData({
        avatarUrl: profile.avatar,
        blobUrl: null,
        thumbhash: profile.avatar_thumbhash,
      });
    }

    if (externalPayload?.uploaded_banner) {
      const blobUrl = URL.createObjectURL(externalPayload.uploaded_banner);
      setBannerData((prev) => ({
        ...prev,
        blobUrl: blobUrl,
        bannerUrl: null,
        thumbhash: null,
        previewBlobUrl: externalPayload.banner_preview_blob_url || blobUrl,
      }));
    } else if (externalPayload?.remove_banner) {
      setBannerData((prev) => ({
        ...prev,
        blobUrl: null,
        bannerUrl: null,
        thumbhash: null,
        previewBlobUrl: null,
      }));
    } else {
      setBannerData({
        bannerUrl: profile.banner,
        blobUrl: null,
        thumbhash: profile.banner_thumbhash,
        previewBlobUrl: null,
      });
    }

    if (
      externalPayload?.display_name !== undefined ||
      externalPayload?.display_name === null
    ) {
      setDisplayName(
        externalPayload.display_name.length
          ? externalPayload.display_name
          : profile.username,
      );
    } else {
      setDisplayName(
        profile.display_name ? profile.display_name : profile.username,
      );
    }
    if (externalPayload && externalPayload.quote !== undefined) {
      const extQuote = externalPayload.quote ?? "";
      setQuote(extQuote.length ? extQuote : (profile.quote ?? ""));
    } else {
      setQuote(profile.quote ?? "");
    }

    if (externalPayload && externalPayload.bio !== undefined) {
      const extBio = externalPayload.bio ?? "";
      setBio(extBio.length ? extBio : (profile.bio ?? ""));
    } else {
      setBio(profile.bio ?? "");
    }
  }, [
    externalPayload,
    profile.avatar,
    profile.avatar_thumbhash,
    profile.banner,
    profile.banner_thumbhash,
    profile.display_name,
    profile.username,
    profile.quote,
  ]);

  return (
    <div className={style["profile-body"]} ref={rootRef}>
      <DialogClose className={style["close-button"]}>
        <IoMdClose />
      </DialogClose>
      <BannerBackdrop
        userId={profile.id}
        banner={bannerData.bannerUrl}
        blobUrl={bannerData.previewBlobUrl || bannerData.blobUrl || null}
        thumbhash={bannerData.thumbhash}
      />

      <div className={style["profile-info"]}>
        <Banner
          userId={profile.id}
          banner={bannerData.bannerUrl}
          blobUrl={bannerData.blobUrl}
          thumbhash={bannerData.thumbhash}
          is_owner={initialIsOwner}
          containerRef={rootRef}
          onChange={handleBannerChange}
        />
        <div className={style["header-group"]}>
          <Avatar
            username={profile.username}
            userId={profile.id}
            avatar={avatarData.avatarUrl}
            blobUrl={avatarData.blobUrl}
            thumbhash={avatarData.thumbhash}
            is_owner={initialIsOwner}
            onChange={handleAvatarChange}
            containerRef={rootRef}
          />
          <div className={style["profile-text"]}>
            <DisplayName
              displayName={displayName}
              isOwner={initialIsOwner}
              onChange={handleDisplayNameChange}
            />
            <p className={style["profile-username"]}>@{profile.username}</p>
          </div>
        </div>
        <div className={style["profile-bio"]}>
          <p><strong>Bio</strong></p>
          <div className={style["profile-bio"]}>
            <EditableTextArea
              value={bio}
              maxChars={300}
              placeholder="Enter your bio..."
              isOwner={initialIsOwner}
              onUpdateChange={handleBioChange}
              />
          </div>
        </div>
        <div className={style["profile-stats"]}>{/* interest tag */}</div>
      </div>
      <div className={style["profile-interests"]}>
        <EditableTextArea 
        value={quote} 
        maxChars={256} 
        placeholder="Enter your quote..."
        isOwner={initialIsOwner}
        onUpdateChange={handleQuoteChange}
        showQuoteIcons={true}
        />

        <div className={style["profile-activity"]}>
          <p>
            <strong>{profile.posts_count}</strong> Posts
          </p>
          <p>
            <strong>{profile.followers_count}</strong> Followers
          </p>
          <p>
            <strong>{profile.following_count}</strong> Following
          </p>
        </div>
        <div className={style["recently-post"]}>
          <div className={style["post-container"]}>
            {/* for pinned post */}

            {recentPost.slice(0, 2).map((post) => (
              <Post
                key={post.id}
                {...post}/>
            ))}
          </div>
        </div>
        <div className={style[""]}></div>
      </div>
    </div>
  );
};

export default ViewProfileBody;
