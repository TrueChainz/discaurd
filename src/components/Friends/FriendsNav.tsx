import { useEffect, useState, useRef, LegacyRef } from "react";
import { cva } from "class-variance-authority";
import { FaUserFriends } from "react-icons/fa";
import dynamic from "next/dynamic";
import { useAutoAnimate } from "@formkit/auto-animate/react";
import { signOut, useSession } from "next-auth/react";
const AddFriendModal = dynamic(() => import("../AddFriendModal"), {
  ssr: false,
});

const friendNavList = ["Online", "All", "Pending"];
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
      isLogout: {
        true: "btn-accent text-white hover:bg-accent-focus btn-outline",
      },
    },
  }
);

function FriendsNav() {
  const [activeNav, setActiveNav] = useState("Online");
  const [modalActive, setModalActive] = useState(false);
  const [parent] = useAutoAnimate(/* optional config */);
  const { data: session } = useSession();

  return (
    <div className="flex-1 lg:flex" ref={parent as LegacyRef<HTMLDivElement>}>
      <ul
        className="navbar-start menu menu-horizontal mx-4 min-w-fit flex-shrink-0 flex-nowrap px-1  lg:flex
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
        <AddFriendModal
          setModalActive={setModalActive}
          username={session.user.username}
        />
      )}
      <ul className="ml-auto ">
        <li
          className={cvaFriendNav({
            selected: "Add Friend" === activeNav,
            isLogout: true,
          })}
          onClick={() => signOut()}
        >
          Sign Out
        </li>
      </ul>
    </div>
  );
}

export default FriendsNav;
