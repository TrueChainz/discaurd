import { create } from "zustand";
import { TAddFriendModal } from "../components/Modals/AddFriendModal";
import { TDisplayUser } from "../components/Modals/DisplayUserModal";

type TModalStates = TDisplayUser | TAddFriendModal;

interface TUseModalStore {
  active: string;
  data: any;
  open: ({ selected, data }: TModalStates) => void;
  close: () => void;
}

export const useModal = create<TUseModalStore>((set) => ({
  active: null,
  data: null,
  open: ({ selected, data }: TModalStates) => set({ active: selected }),
  close: () => set({ active: null, data: null }),
}));
