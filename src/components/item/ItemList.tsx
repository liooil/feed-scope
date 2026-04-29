import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Star, Globe, Clock } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { ItemWithState } from "@/lib/types";
import { markRead, toggleStar } from "@/lib/api";
import { cn } from "@/lib/utils";

interface ItemListProps {
  items: ItemWithState[];
  selectedItemId: string | null;
  onSelectItem: (item: ItemWithState) => void;
  onItemsChanged: () => void;
}

function timeAgo(dateStr: string | null): string {
  if (!dateStr) return "";
  const date = new Date(dateStr);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);
  if (seconds < 60) return `${seconds}s`;
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d`;
  return date.toLocaleDateString();
}

function feedKindColor(kind: string): string {
  switch (kind) {
    case "jira":
      return "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200";
    case "gitlab":
      return "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200";
    default:
      return "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200";
  }
}

export function ItemList({ items, selectedItemId, onSelectItem, onItemsChanged }: ItemListProps) {
  const handleToggleStar = async (e: React.MouseEvent, itemId: string) => {
    e.stopPropagation();
    try {
      await toggleStar(itemId);
      onItemsChanged();
    } catch (err) {
      console.error("Failed to toggle star:", err);
    }
  };

  const handleSelect = async (item: ItemWithState) => {
    onSelectItem(item);
    if (!item.state.read) {
      try {
        await markRead(item.id);
        onItemsChanged();
      } catch (err) {
        console.error("Failed to mark read:", err);
      }
    }
  };

  if (items.length === 0) {
    return (
      <div className="flex flex-1 items-center justify-center text-sm text-muted-foreground">
        <div className="text-center">
          <Globe className="mx-auto mb-2 h-8 w-8 opacity-40" />
          <p>No items yet</p>
          <p className="text-xs">Add a feed and sync to see items here.</p>
        </div>
      </div>
    );
  }

  return (
    <ScrollArea className="flex-1">
      <div>
        {items.map((item) => (
          <div key={item.id}>
            <button
              className={cn(
                "w-full px-4 py-3 text-left transition-colors hover:bg-accent/50",
                selectedItemId === item.id && "bg-accent",
                !item.state.read && "border-l-2 border-l-primary",
              )}
              onClick={() => handleSelect(item)}
            >
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <span
                      className={cn(
                        "text-xs font-medium",
                        !item.state.read ? "text-foreground" : "text-muted-foreground",
                      )}
                    >
                      {item.feed_name}
                    </span>
                    <Badge
                      variant="outline"
                      className={cn("text-[10px] px-1 py-0 font-normal", feedKindColor(item.feed_kind))}
                    >
                      {item.feed_kind}
                    </Badge>
                  </div>
                  <p
                    className={cn(
                      "mt-0.5 text-sm line-clamp-2",
                      !item.state.read ? "font-medium text-foreground" : "text-muted-foreground",
                    )}
                  >
                    {item.title}
                  </p>
                  {item.summary && (
                    <p className="mt-0.5 text-xs text-muted-foreground line-clamp-1">
                      {item.summary}
                    </p>
                  )}
                </div>
                <div className="flex shrink-0 flex-col items-end gap-1">
                  <span className="text-[11px] text-muted-foreground">
                    <Clock className="mr-0.5 inline h-3 w-3" />
                    {timeAgo(item.published_at)}
                  </span>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-5 w-5"
                    onClick={(e) => handleToggleStar(e, item.id)}
                  >
                    <Star
                      className={cn(
                        "h-3.5 w-3.5",
                        item.state.starred
                          ? "fill-yellow-400 text-yellow-400"
                          : "text-muted-foreground/40",
                      )}
                    />
                  </Button>
                </div>
              </div>
            </button>
            <Separator />
          </div>
        ))}
      </div>
    </ScrollArea>
  );
}
