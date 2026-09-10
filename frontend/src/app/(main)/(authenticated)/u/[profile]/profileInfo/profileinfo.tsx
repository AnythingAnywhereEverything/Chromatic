import { PublicUserProfileResponse } from "@/api/user/profile";

import style from "./style.module.scss";
import { FaQuoteLeft, FaQuoteRight } from "react-icons/fa";

export const ProfileInfo = ({
    profile,
    isOwner,
}: {
    profile: PublicUserProfileResponse;
    isOwner: boolean;
}) => {
    return (
        <div className={style["info-wrapper"]}>
            <div className={style["profile-info"]}>
                <div className={style["profile-section-group"]}>
                    <h3>Quote</h3>
                    <div className={style["profile-quote"]}>
                        <div className={style["profile-quote-icon"]}>
                            <FaQuoteLeft />
                        </div>

                        {profile.quote ? (
                            <p>{profile.quote}</p>
                        ) : (
                            <p style={{ color: "var(--text-muted)" }}>
                                This user has not set a quote yet.
                            </p>
                        )}
                        <div className={style["profile-quote-icon"]}>
                            <FaQuoteRight />
                        </div>
                    </div>
                </div>
                <div className={style["profile-section-group"]}>
                    <h3>Bio</h3>
                    <div className={style["profile-bio"]}>
                        {profile.bio ? (
                            <p>{profile.bio}</p>
                        ) : (
                            <p style={{ color: "var(--text-muted)" }}>
                                This user has not set a bio yet.
                            </p>
                        )}
                    </div>
                </div>

                <div className={style["profile-section-group"]}>
                    <h3>Interested in</h3>
                    <div className={style["profile-additional-info"]}>
                        Working in progress...
                    </div>
                </div>
            </div>
        </div>
    );
};
