import { useEffect, useState, useRef, LegacyRef } from "react";
import { cva } from "class-variance-authority";
import { FaUserFriends } from "react-icons/fa";
import dynamic from "next/dynamic";
import { useAutoAnimate } from "@formkit/auto-animate/react";
const AddFriendModal = dynamic(() => import("../AddFriendModal"), {
  ssr: false,
});

const cvaFriendNav = cva(
  "btn-ghost btn-sm btn rounded-lg normal-case hover:bg-zinc-600 mx-2",
  {
    variants: {
      selected: {
        true: "bg-base-200",
      },
      isAdd: {
        true: "bg-accent text-white hover:bg-accent-focus",
      },
    },
  }
);

const friendNavList = ["Online", "All", "Pending"];

function Friends() {
  const [activeNav, setActiveNav] = useState("Online");
  const [modalActive, setModalActive] = useState(false);
  const [parent] = useAutoAnimate(/* optional config */);

  console.log(modalActive);
  return (
    <div className="navbar border-b-2 border-b-zinc-600  bg-base-100">
      <div
        className="navbar-start  lg:flex"
        ref={parent as LegacyRef<HTMLDivElement>}
      >
        <div className=" flex justify-start border-r-2 border-slate-600 py-0 px-2 text-xl normal-case ">
          <FaUserFriends size={24} className="mr-2" />
          <h1 className="text-lg font-bold ">Friends</h1>
        </div>
        <ul
          className="0 menu menu-horizontal mx-4 min-w-fit flex-shrink-0 flex-nowrap  px-1
        "
        >
          {friendNavList.map((nav, i) => {
            return (
              <li
                className={cvaFriendNav({
                  selected: nav === activeNav,
                  isAdd: false,
                })}
                onClick={() => {
                  setActiveNav(nav);
                }}
              >
                {nav}
              </li>
            );
          })}
          <label
            className={cvaFriendNav({
              selected: "Add Friend" === activeNav,
              isAdd: true,
            })}
            onClick={() => setModalActive(true)}
          >
            Add Friend
          </label>
        </ul>
        {modalActive === true && (
          <AddFriendModal setModalActive={setModalActive} />
        )}
      </div>
    </div>
  );
}

export default Friends;
