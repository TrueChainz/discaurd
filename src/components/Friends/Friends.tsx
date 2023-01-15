import { useEffect, useState, useRef, LegacyRef } from "react";
import { cva } from "class-variance-authority";
import { FaUserFriends } from "react-icons/fa";
import dynamic from "next/dynamic";
import { useAutoAnimate } from "@formkit/auto-animate/react";
import FriendsNav from "./FriendsNav";
const AddFriendModal = dynamic(() => import("../AddFriendModal"), {
  ssr: false,
});

function Friends() {
  const [activeNav, setActiveNav] = useState("Online");
  const [modalActive, setModalActive] = useState(false);
  const [parent] = useAutoAnimate(/* optional config */);

  return (
    <div className="navbar border-b-2 border-b-zinc-600  bg-base-100">
      <div className=" flex justify-start border-r-2 border-slate-600 py-0 px-2 text-xl normal-case ">
        <FaUserFriends size={24} className="mr-2" />
        <h1 className="text-lg font-bold ">Friends</h1>
      </div>
      <FriendsNav />
    </div>
  );
}

export default Friends;
