import { cva } from "class-variance-authority";
import { useState } from "react";
import { FaUserFriends } from "react-icons/fa";
import Friends from "../components/Friends/Friends";

const headerNavItem = cva(
  "btn-ghost btn flex cursor-pointer justify-start rounded-md py-2 px-0 text-xl normal-case hover:bg-zinc-600",
  {
    variants: {
      selected: {
        true: "bg-slate-500 ",
      },
    },
  }
);

const friendDirect = cva(
  "btn-ghost btn flex cursor-pointer items-center justify-start rounded-md my-2 px-0 normal-case hover:bg-zinc-600",
  {
    variants: {
      selected: {
        true: "bg-slate-500",
      },
    },
  }
);

function App() {
  const [selectedNavigation, setSelectedNavigation] = useState();
  return (
    <div className="flex h-screen">
      <section className="navbar-start h-full w-60 flex-shrink-0 bg-base-300 p-2">
        <div className={headerNavItem({ selected: false })}>
          <FaUserFriends size={24} className="mx-4" />
          <h1 className="text-lg font-bold">
            <a className="">Friends</a>
          </h1>
        </div>
        <br />
        <h2 className="mx-2 text-xs font-bold uppercase -tracking-normal">
          Direct Messages
        </h2>

        <div className={friendDirect({ selected: false })}>
          <div className="mx-2 h-8 w-8 rounded-full bg-base-100"></div>
          <h2 className="text-md font-bold">
            <a>Friend_1</a>
          </h2>
        </div>
        <div className={friendDirect({ selected: false })}>
          <div className="mx-2 h-8 w-8 rounded-full bg-base-100"></div>
          <h2 className="text-md font-bold">
            <a>Friend_3</a>
          </h2>
        </div>
        <div className={friendDirect({ selected: false })}>
          <div className="mx-2 h-8 w-8 rounded-full bg-base-100"></div>
          <h2 className="text-md font-bold">
            <a>Friend_2</a>
          </h2>
        </div>
        <div className={friendDirect({ selected: false })}>
          <div className="mx-2 h-8 w-8 rounded-full bg-base-100"></div>
          <h2 className="text-md font-bold">
            <a>Friend_3</a>
          </h2>
        </div>
      </section>
      <section className="w-full flex-1">
        <Friends />
      </section>

      {/* Put this part before </body> tag */}
    </div>
  );
}

export default App;
