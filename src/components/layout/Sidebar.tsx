import { useState } from "react";
import { Plus, Rss, Settings, Trash2, RefreshCw, AlertCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Badge } from "@/components/ui/badge";
import { FeedDialog } from "@/components/feed/FeedDialog";
import { SettingsDialog } from "@/features/settings/SettingsDialog";
import { deleteFeed, syncFeed } from "@/lib/api";
import type { Feed } from "@/lib/types";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

interface SidebarProps {
  feeds: Feed[];
  selectedFeedId: string | null;
  onSelectFeed: (id: string | null) => void;
  onFeedsChanged: () => void;
}

export function Sidebar({ feeds, selectedFeedId, onSelectFeed, onFeedsChanged }: SidebarProps) {
  const [feedDialogOpen, setFeedDialogOpen] = useState(false);
  const [editingFeed, setEditingFeed] = useState<Feed | null>(null);
  const [deleteConfirmId, setDeleteConfirmId] = useState<string | null>(null);
  const [syncingId, setSyncingId] = useState<string | null>(null);
  const [settingsOpen, setSettingsOpen] = useState(false);

  const handleDelete = async (id: string) => {
    try {
      await deleteFeed(id);
      setDeleteConfirmId(null);
      if (selectedFeedId === id) onSelectFeed(null);
      onFeedsChanged();
    } catch (e) {
      console.error("Failed to delete feed:", e);
    }
  };

  const handleSync = async (id: string) => {
    setSyncingId(id);
    try {
      await syncFeed(id);
      onFeedsChanged();
    } catch (e) {
      console.error("Failed to sync feed:", e);
    } finally {
      setSyncingId(null);
    }
  };

  const categories = [...new Set(feeds.map((f) => f.category))];

  return (
    <div className="flex w-64 flex-col border-r bg-sidebar">
      <div className="flex items-center justify-between p-3">
        <h1 className="text-sm font-semibold text-sidebar-foreground">Feed Scope</h1>
        <Button
          variant="ghost"
          size="icon"
          className="h-7 w-7"
          onClick={() => {
            setEditingFeed(null);
            setFeedDialogOpen(true);
          }}
        >
          <Plus className="h-4 w-4" />
        </Button>
      </div>
      <Separator />
      <ScrollArea className="flex-1">
        <div className="p-2">
          <button
            className={`w-full rounded-md px-2 py-1.5 text-left text-sm transition-colors hover:bg-sidebar-accent ${
              selectedFeedId === null ? "bg-sidebar-accent text-sidebar-accent-foreground" : "text-sidebar-foreground"
            }`}
            onClick={() => onSelectFeed(null)}
          >
            <div className="flex items-center justify-between">
              <span>All Feeds</span>
              <Badge variant="secondary" className="text-[10px] px-1.5 py-0">
                {feeds.filter((f) => f.enabled).length}
              </Badge>
            </div>
          </button>

          {categories.map((cat) => (
            <div key={cat} className="mt-3">
              <p className="px-2 text-[11px] font-medium uppercase text-muted-foreground">
                {cat}
              </p>
              {feeds
                .filter((f) => f.category === cat)
                .map((feed) => (
                  <DropdownMenu key={feed.id}>
                    <DropdownMenuTrigger asChild>
                      <button
                        className={`group w-full rounded-md px-2 py-1.5 text-left text-sm transition-colors hover:bg-sidebar-accent ${
                          selectedFeedId === feed.id
                            ? "bg-sidebar-accent text-sidebar-accent-foreground"
                            : "text-sidebar-foreground"
                        }`}
                        onClick={() => onSelectFeed(feed.id)}
                      >
                        <div className="flex items-center gap-2">
                          <Rss className="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
                          <span className="truncate flex-1">{feed.name}</span>
                          {!feed.enabled && (
                            <div className="h-2 w-2 rounded-full bg-muted-foreground/40" />
                          )}
                          {feed.last_error && (
                            <TooltipProvider>
                              <Tooltip>
                                <TooltipTrigger asChild>
                                  <span className="inline-flex shrink-0">
                                    <AlertCircle className="h-3.5 w-3.5 text-destructive" />
                                  </span>
                                </TooltipTrigger>
                                <TooltipContent side="right" className="max-w-[240px] text-xs">
                                  {feed.last_error}
                                </TooltipContent>
                              </Tooltip>
                            </TooltipProvider>
                          )}
                        </div>
                      </button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent className="w-40">
                      <DropdownMenuItem
                        onClick={() => handleSync(feed.id)}
                        disabled={syncingId === feed.id}
                      >
                        <RefreshCw className={`mr-2 h-3.5 w-3.5 ${syncingId === feed.id ? "animate-spin" : ""}`} />
                        Sync now
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        onClick={() => {
                          setEditingFeed(feed);
                          setFeedDialogOpen(true);
                        }}
                      >
                        Edit
                      </DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem
                        className="text-destructive"
                        onClick={() => setDeleteConfirmId(feed.id)}
                      >
                        <Trash2 className="mr-2 h-3.5 w-3.5" />
                        Delete
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                ))}
            </div>
          ))}

          {feeds.length === 0 && (
            <p className="px-2 pt-4 text-xs text-muted-foreground">
              No feeds yet. Click + to add your first feed.
            </p>
          )}
        </div>
      </ScrollArea>
      <Separator />
      <div className="p-2">
        <button
          className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent"
          onClick={() => setSettingsOpen(true)}
        >
          <Settings className="h-4 w-4" />
          Settings
        </button>
      </div>

      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />

      <FeedDialog
        open={feedDialogOpen}
        onOpenChange={(open) => {
          setFeedDialogOpen(open);
          if (!open) setEditingFeed(null);
        }}
        feed={editingFeed}
        onSaved={onFeedsChanged}
      />

      <Dialog open={deleteConfirmId !== null} onOpenChange={() => setDeleteConfirmId(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Feed</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete this feed? All associated items will also be removed.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteConfirmId(null)}>
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={() => deleteConfirmId && handleDelete(deleteConfirmId)}
            >
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
