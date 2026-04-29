import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { ExternalLink, X, Star, Globe } from "lucide-react";
import type { ItemWithState } from "@/lib/types";
import { markRead, markUnread, toggleStar } from "@/lib/api";
import { cn } from "@/lib/utils";

interface ItemDetailProps {
  item: ItemWithState | null;
  onClose: () => void;
  onItemsChanged: () => void;
}

export function ItemDetail({ item, onClose, onItemsChanged }: ItemDetailProps) {
  if (!item) {
    return (
      <div className="flex flex-1 items-center justify-center text-sm text-muted-foreground">
        <div className="text-center">
          <Globe className="mx-auto mb-2 h-8 w-8 opacity-40" />
          <p>Select an item to view details</p>
        </div>
      </div>
    );
  }

  const handleToggleRead = async () => {
    try {
      if (item.state.read) {
        await markUnread(item.id);
      } else {
        await markRead(item.id);
      }
      onItemsChanged();
    } catch (e) {
      console.error("Failed to toggle read:", e);
    }
  };

  const handleToggleStar = async () => {
    try {
      await toggleStar(item.id);
      onItemsChanged();
    } catch (e) {
      console.error("Failed to toggle star:", e);
    }
  };

  const handleOpenLink = () => {
    if (item.link) {
      // Use Tauri opener plugin
      import("@tauri-apps/plugin-shell").then(({ open }) => {
        open(item.link!);
      }).catch(() => {
        window.open(item.link!, "_blank");
      });
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between border-b px-4 py-2">
        <div className="flex items-center gap-2">
          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={onClose}>
            <X className="h-4 w-4" />
          </Button>
          <Badge variant="outline" className="text-[10px]">
            {item.feed_kind}
          </Badge>
        </div>
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={handleToggleStar}>
            <Star
              className={cn(
                "h-4 w-4",
                item.state.starred ? "fill-yellow-400 text-yellow-400" : "text-muted-foreground",
              )}
            />
          </Button>
          <Button variant="ghost" size="icon" className="h-7 w-7" onClick={handleToggleRead}>
            <span className="text-xs">{item.state.read ? "Unread" : "Read"}</span>
          </Button>
          {item.link && (
            <Button variant="ghost" size="icon" className="h-7 w-7" onClick={handleOpenLink}>
              <ExternalLink className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>

      <ScrollArea className="flex-1">
        <div className="p-6">
          <h2 className="text-lg font-semibold leading-snug">{item.title}</h2>
          <div className="mt-2 flex items-center gap-3 text-xs text-muted-foreground">
            <span>{item.feed_name}</span>
            {item.author && <span>by {item.author}</span>}
            {item.published_at && (
              <span>{new Date(item.published_at).toLocaleString()}</span>
            )}
          </div>

          <Separator className="my-4" />

          {item.summary && (
            <div className="mb-4 rounded-md bg-muted/50 p-3 text-sm">
              {item.summary}
            </div>
          )}

          {item.content && (
            <div
              className="prose prose-sm dark:prose-invert max-w-none text-sm"
              dangerouslySetInnerHTML={{
                __html: item.content,
              }}
            />
          )}

          {!item.content && !item.summary && (
            <p className="text-sm text-muted-foreground">No content available.</p>
          )}
        </div>
      </ScrollArea>
    </div>
  );
}
