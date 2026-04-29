import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { RefreshCw, Filter } from "lucide-react";
import type { Feed } from "@/lib/types";

interface TopBarProps {
  onlyUnread: boolean;
  onlyStarred: boolean;
  onToggleUnread: () => void;
  onToggleStarred: () => void;
  onRefresh: () => void;
  selectedFeed: Feed | null;
}

export function TopBar({
  onlyUnread,
  onlyStarred,
  onToggleUnread,
  onToggleStarred,
  onRefresh,
  selectedFeed,
}: TopBarProps) {
  return (
    <div className="flex items-center gap-2 border-b px-4 py-2">
      <h2 className="text-sm font-medium">
        {selectedFeed ? selectedFeed.name : "All Feeds"}
      </h2>
      {selectedFeed?.last_error && (
        <Badge variant="destructive" className="text-[10px]">
          Error
        </Badge>
      )}
      <div className="flex-1" />
      <div className="flex items-center gap-1">
        <Button
          variant={onlyUnread ? "secondary" : "ghost"}
          size="sm"
          className="h-7 text-xs"
          onClick={onToggleUnread}
        >
          <Filter className="mr-1 h-3 w-3" />
          Unread
        </Button>
        <Button
          variant={onlyStarred ? "secondary" : "ghost"}
          size="sm"
          className="h-7 text-xs"
          onClick={onToggleStarred}
        >
          Starred
        </Button>
      </div>
      <Separator orientation="vertical" className="h-5" />
      <Button variant="ghost" size="icon" className="h-7 w-7" onClick={onRefresh}>
        <RefreshCw className="h-3.5 w-3.5" />
      </Button>
    </div>
  );
}
