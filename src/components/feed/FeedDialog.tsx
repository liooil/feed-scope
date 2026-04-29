import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { Feed, FeedKind } from "@/lib/types";
import { addFeed, updateFeed } from "@/lib/api";

interface FeedDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  feed: Feed | null;
  onSaved: () => void;
}

const FEED_KINDS: { value: FeedKind; label: string }[] = [
  { value: "rss", label: "RSS" },
  { value: "atom", label: "Atom" },
  { value: "json", label: "JSON Feed" },
  { value: "jira", label: "Jira" },
  { value: "gitlab", label: "GitLab" },
];

export function FeedDialog({ open, onOpenChange, feed, onSaved }: FeedDialogProps) {
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");
  const [kind, setKind] = useState<FeedKind>("rss");
  const [category, setCategory] = useState("uncategorized");
  const [enabled, setEnabled] = useState(true);
  const [refreshInterval, setRefreshInterval] = useState(300);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (feed) {
      setName(feed.name);
      setUrl(feed.url);
      setKind(feed.kind);
      setCategory(feed.category);
      setEnabled(feed.enabled);
      setRefreshInterval(feed.refresh_interval_seconds);
    } else {
      setName("");
      setUrl("");
      setKind("rss");
      setCategory("uncategorized");
      setEnabled(true);
      setRefreshInterval(300);
    }
  }, [feed, open]);

  const handleSave = async () => {
    if (!name.trim() || !url.trim()) return;
    setSaving(true);
    try {
      if (feed) {
        await updateFeed({
          ...feed,
          name: name.trim(),
          url: url.trim(),
          kind,
          category,
          enabled,
          refresh_interval_seconds: refreshInterval,
        });
      } else {
        await addFeed({
          name: name.trim(),
          url: url.trim(),
          kind,
          category,
          enabled,
          refresh_interval_seconds: refreshInterval,
          credential_id: null,
        });
      }
      onSaved();
      onOpenChange(false);
    } catch (e) {
      console.error("Failed to save feed:", e);
    } finally {
      setSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{feed ? "Edit Feed" : "Add Feed"}</DialogTitle>
          <DialogDescription>
            {feed ? "Update the feed settings below." : "Enter the details for your new feed."}
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-2">
          <div className="grid gap-2">
            <Label htmlFor="name">Name</Label>
            <Input
              id="name"
              placeholder="My Feed"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="url">URL</Label>
            <Input
              id="url"
              placeholder="https://example.com/feed.xml"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
            />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="grid gap-2">
              <Label>Feed Type</Label>
              <Select value={kind} onValueChange={(v) => setKind(v as FeedKind)}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {FEED_KINDS.map((k) => (
                    <SelectItem key={k.value} value={k.value}>
                      {k.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="category">Category</Label>
              <Input
                id="category"
                placeholder="uncategorized"
                value={category}
                onChange={(e) => setCategory(e.target.value)}
              />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="grid gap-2">
              <Label htmlFor="interval">Refresh (seconds)</Label>
              <Input
                id="interval"
                type="number"
                min={60}
                value={refreshInterval}
                onChange={(e) => setRefreshInterval(Number(e.target.value))}
              />
            </div>
            <div className="flex items-end gap-2 pb-2">
              <Switch id="enabled" checked={enabled} onCheckedChange={setEnabled} />
              <Label htmlFor="enabled">Enabled</Label>
            </div>
          </div>
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={saving || !name.trim() || !url.trim()}>
            {saving ? "Saving..." : "Save"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
