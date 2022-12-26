import { useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

const Home = () => {
  const [name, setName] = createSignal("");
  const [isLoggedIn, setIsLoggedIn] = createSignal(false);
  const navigate = useNavigate();

  if (!isLoggedIn()) navigate("login");

  return (
    <div class="w-screen h-screen bg-gray-700 flex">
      <div class="w-20 bg-gray-700 h-full"></div>
      <div class="w-56 bg-gray-600">
        <div></div>
      </div>
      <div class="flex-grow bg-gray-700">
        <div class="h-8 flex gap-4 text-slate-400 font-medium m-2 ">
          <div class="font-bold p-1 hover:bg-slate-400 hover:bg-opacity-20 rounded-md cursor-pointer">
            Friends
          </div>
          <div class="p-1 hover:bg-slate-400 hover:bg-opacity-20 rounded-md cursor-pointer">
            Online
          </div>
          <div class="p-1 hover:bg-slate-400 hover:bg-opacity-20 rounded-md cursor-pointer">
            All
          </div>
          <div class="p-1 hover:bg-slate-400 hover:bg-opacity-20 rounded-md cursor-pointer">
            Pending
          </div>
          <div class="p-1 text-slate-300 hover:bg-opacity-80 rounded-md cursor-pointer bg-green-600">
            Add Friend
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;
