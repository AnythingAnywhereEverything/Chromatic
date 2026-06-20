import Post from "@components/post";
import PostBox from "@components/postbox";
import React from "react";
import { useParams } from "react-router-dom";

export default function Test() {
  const { id } = useParams<{ id: string }>();

  return (
    <div>
      <h1>Test Page</h1>
      <p>You are viewing test ID: {id}</p>
      <PostBox/>
      <Post/>
    </div>
  );
}
