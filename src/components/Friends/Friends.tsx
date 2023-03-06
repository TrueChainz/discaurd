import { FaUserFriends } from "react-icons/fa";
import dynamic from "next/dynamic";
import { useAutoAnimate } from "@formkit/auto-animate/react";
import FriendsNav from "./FriendsNav";
import { useFriendStore, ViewState } from "../../store/friendStore";
import { TFriend } from "../../lib/services";
import { AiOutlineClose, AiOutlineCheck } from "react-icons/ai";
import { BsFillChatTextFill } from "react-icons/bs";
import { BiMenu } from "react-icons/bi";

interface TFriendRowProps extends TFriend {
  view: ViewState;
}

function FriendRow(props: TFriendRowProps) {
  console.log(props);

  const getIcons = () => {
    switch (props.status) {
      case "Friend":
        return (
          <>
            <div className="mx-2 rounded-full bg-base-300 hover:bg-base-100 ">
              <BsFillChatTextFill className="m-2" size={16} />
            </div>
            <div className="mx-1 rounded-full bg-base-300 hover:bg-base-100 ">
              <BiMenu className="m-2" size={16} />
            </div>
          </>
        );
      case "Incoming":
        return (
          <>
            <div className="mx-2 rounded-full bg-base-300 hover:bg-base-100 ">
              <AiOutlineCheck className="m-2" size={18} />
            </div>
            <div className="mx-1 rounded-full bg-base-300 hover:bg-base-100 ">
              <AiOutlineClose className="m-2" size={18} />
            </div>
          </>
        );

      default:
        return null;
    }
  };
  return (
    <div className="my-2 flex cursor-pointer items-center justify-start rounded-md bg-base-300 py-2 px-2 normal-case hover:bg-base-200">
      <div className="mx-2 h-8 w-8 rounded-full bg-base-100"></div>
      <h2 className="text-md font-bold">
        <a>{props.username}</a>
      </h2>
      <div className="ml-auto flex items-center px-2">{getIcons()}</div>
    </div>
  );
}

function Friends() {
  const { all, online, pending, active, fetchFriends } = useFriendStore(
    (state) => state
  );
  return (
    <div className="px-4">
      <div className="navbar border-b-2 border-b-zinc-600  bg-base-100">
        <div className=" flex justify-start border-r-2 border-slate-600 py-0 px-2 text-xl normal-case ">
          <FaUserFriends size={24} className="mr-2" />
          <h1 className="text-lg font-bold ">Friends</h1>
        </div>
        <FriendsNav />
      </div>
      <h1 className="btn" onClick={() => fetchFriends("TrueChainz")}>
        FETCH PENDING FRIENDS
      </h1>
      {active === ViewState.Online &&
        online.map((friend) => {
          return <FriendRow key={friend.id} {...friend} view={active} />;
        })}
      {active === ViewState.All &&
        all.map((friend) => {
          return <FriendRow key={friend.id} {...friend} view={active} />;
        })}
      {active === ViewState.Pending &&
        pending.map((friend) => {
          return <FriendRow key={friend.id} {...friend} view={active} />;
        })}
    </div>
  );
}

export default Friends;
