"use client";

import {
    FocusedPostProvider,
    FocusedPostView,
} from "./_components/focusedPost";

function PostContentPage({ params }: { params: { post: string } }) {
    return (
        <FocusedPostProvider postId={params.post}>
            <FocusedPostView />
        </FocusedPostProvider>
    );
}

export default PostContentPage;