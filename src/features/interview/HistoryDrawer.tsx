import type { ReactNode } from "react";
import { Dialog } from "@astryxdesign/core/Dialog";
import { DialogHeader } from "@astryxdesign/core/Dialog";

/** Astryx owns modal focus, Escape/backdrop dismissal and focus restoration. */
export function HistoryDrawer({
  open,
  onOpenChange,
  title,
  children,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  children: ReactNode;
}) {
  return (
    <Dialog
      isOpen={open}
      onOpenChange={onOpenChange}
      purpose="info"
      position={{ start: 0, top: 0, bottom: 0 }}
      width="min(400px, 90vw)"
      maxHeight="100dvh"
      className="history-drawer"
    >
      <DialogHeader title={title} onOpenChange={onOpenChange} />
      <div className="history-drawer-content">{children}</div>
    </Dialog>
  );
}
