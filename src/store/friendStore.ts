import { create } from "zustand";
import { showPendingFriends, TFriend } from "../lib/services";

export enum ViewState {
  All = "All",
  Online = "Online",
  Pending = "Pending",
}

interface TUseFriendStore {
  all: TFriend[];
  online: TFriend[];
  pending: TFriend[];
  active: ViewState;
  setActive: (selected: ViewState) => void;
  fetchFriends: (username: string) => void;
}

export const useFriendStore = create<TUseFriendStore>((set) => ({
  all: [{ id: "1", username: "lioli", status: "Friend" }],
  online: [{ id: "1", username: "lioli", status: "Friend" }],
  pending: [{ id: "1", username: "lioli", status: "Friend" }],
  active: ViewState.All,
  setActive: (selected: ViewState) => set({ active: selected }),
  fetchFriends: async (username: string) => {
    try {
      const pending = await showPendingFriends(username);
      if (!pending.success) throw Error(pending.error_message);

      return set({ pending: pending.friends });
    } catch (err) {
      console.log(err.message);
    }
  },
}));
