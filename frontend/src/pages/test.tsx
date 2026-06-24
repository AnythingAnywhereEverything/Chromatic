import Post from "@components/post";
import PostBox from "@components/createPost";
import React from "react";
import { useParams } from "react-router-dom";
import { PostProp } from "@components/post";
import { BottomPost } from "@components/ui/postComp";

const Posts: PostProp[] = [
    {
        postid: "post_001",
        ownerid: "user_123",
        ownerName: "Alice Johnson",
        profileImg: "https://picsum.photos/seed/alice/200/200",
        text: "Had an amazing time hiking this weekend!",
        images: [
            "https://picsum.photos/seed/hike1/800/600",
            "https://picsum.photos/seed/hike2/800/600",
            "https://picsum.photos/seed/hike2/800/600",
        ],
        like: 10000,
        comments: ["hello wassp", "eyy"],
        repost: 12,
    },
    {
        postid: "post_002",
        ownerid: "1",
        ownerName: "Michael Chen",
        profileImg: "https://picsum.photos/seed/alice/200/200",
        text: `Morning light spilled across the quiet street.
            A small bird perched on the fence and sang.
            People hurried by with coffee in their hands, some rushing to catch a bus while others strolled leisurely through the neighborhood.
            The breeze carried the scent of fresh rain mixed with the aroma of bread from a nearby bakery.
            Clouds drifted slowly across the pale blue sky as birds gathered on rooftops and telephone wires.
            Shopkeepers unlocked their doors, arranged displays, and greeted familiar faces with warm smiles.
            Children laughed in a nearby park while parents watched from shaded benches.
            As the sun climbed higher, golden light reflected from windows and puddles left behind by the rain.
            By evening, neighbors exchanged greetings, street lamps flickered to life, and the day ended with a calm and steady rhythm that reminded everyone to appreciate ordinary moments.`,
        like : 2,
        comments: [],
        repost: 0
    },
    {
        postid: "post_002",
        ownerid: "1",
        ownerName: "Michael Chen",
        profileImg: "https://picsum.photos/seed/alice/200/200",
        text: `Morning light spilled across the quiet street.
            A small bird perched on the fence and sang.
            People hurried by with coffee in their hands, some rushing to catch a bus while others strolled leisurely through the neighborhood.
            The breeze carried the scent of fresh rain mixed with the aroma of bread from a nearby bakery.
            Clouds drifted slowly across the pale blue sky as birds gathered on rooftops and telephone wires.
            Shopkeepers unlocked their doors, arranged displays, and greeted familiar faces with warm smiles.
            Children laughed in a nearby park while parents watched from shaded benches.
            As the sun climbed higher, golden light reflected from windows and puddles left behind by the rain.
            By evening, neighbors exchanged greetings, street lamps flickered to life, and the day ended with a calm and steady rhythm that reminded everyone to appreciate ordinary moments.`,
        like : 2,
        comments: [],
        repost: 0
    },
];
export default function Test() {
  const { id } = useParams<{ id: string }>();

  return (
    <div>
      <h1>Test Page</h1>
      <p>You are viewing test ID: {id}</p>
      <PostBox/>
      {Posts.map((post) => (
          <Post
              postid={post.postid}
              ownerid={post.ownerid}
              ownerName={post.ownerName}
              profileImg={post.profileImg}
              text={post.text}
              images={post.images}
              like={post.like}
              comments={post.comments}
              repost={post.repost}
          />
      ))}
    </div>
  );
}
