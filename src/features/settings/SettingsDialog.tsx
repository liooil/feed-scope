import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Separator } from "@/components/ui/separator";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import type { AppSettings } from "@/lib/types";
import { getSettings, saveSettings } from "@/lib/api";

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const [settings, setSettings] = useState<AppSettings>({
    sync_interval_default: 300,
    notifications_enabled: true,
    quiet_hours_start: null,
    quiet_hours_end: null,
    max_items_per_feed: 500,
  });
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (open) {
      getSettings()
        .then(setSettings)
        .catch((e) => console.error("Failed to load settings:", e));
    }
  }, [open]);

  const handleSave = async () => {
    setSaving(true);
    try {
      await saveSettings(settings);
      onOpenChange(false);
    } catch (e) {
      console.error("Failed to save settings:", e);
    } finally {
      setSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[460px]">
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
          <DialogDescription>Configure app-wide preferences.</DialogDescription>
        </DialogHeader>
        <div className="grid gap-5 py-2">
          <div className="grid gap-3">
            <h4 className="text-sm font-medium">Sync</h4>
            <div className="grid grid-cols-2 gap-4">
              <div className="grid gap-2">
                <Label htmlFor="syncInterval">Default refresh interval (s)</Label>
                <Input
                  id="syncInterval"
                  type="number"
                  min={60}
                  value={settings.sync_interval_default}
                  onChange={(e) =>
                    setSettings({ ...settings, sync_interval_default: Number(e.target.value) })
                  }
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="maxItems">Max items per feed</Label>
                <Input
                  id="maxItems"
                  type="number"
                  min={50}
                  value={settings.max_items_per_feed}
                  onChange={(e) =>
                    setSettings({ ...settings, max_items_per_feed: Number(e.target.value) })
                  }
                />
              </div>
            </div>
          </div>

          <Separator />

          <div className="grid gap-3">
            <h4 className="text-sm font-medium">Notifications</h4>
            <div className="flex items-center justify-between">
              <div className="grid gap-0.5">
                <Label htmlFor="notifsEnabled">Enable notifications</Label>
                <p className="text-xs text-muted-foreground">
                  Show desktop notifications for matching items
                </p>
              </div>
              <Switch
                id="notifsEnabled"
                checked={settings.notifications_enabled}
                onCheckedChange={(v) =>
                  setSettings({ ...settings, notifications_enabled: v })
                }
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div className="grid gap-2">
                <Label htmlFor="quietStart">Quiet hours start</Label>
                <Input
                  id="quietStart"
                  type="time"
                  value={settings.quiet_hours_start ?? ""}
                  onChange={(e) =>
                    setSettings({
                      ...settings,
                      quiet_hours_start: e.target.value || null,
                    })
                  }
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="quietEnd">Quiet hours end</Label>
                <Input
                  id="quietEnd"
                  type="time"
                  value={settings.quiet_hours_end ?? ""}
                  onChange={(e) =>
                    setSettings({
                      ...settings,
                      quiet_hours_end: e.target.value || null,
                    })
                  }
                />
              </div>
            </div>
          </div>
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={saving}>
            {saving ? "Saving..." : "Save"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
