import { useState, useEffect, useCallback } from "react";
import { Sidebar } from "@/components/layout/Sidebar";
import { ItemList } from "@/components/item/ItemList";
import { ItemDetail } from "@/components/item/ItemDetail";
import { TopBar } from "@/components/layout/TopBar";
import type { Feed, ItemWithState } from "@/lib/types";
import { getFeeds, getItems } from "@/lib/api";

function App() {
  const [feeds, setFeeds] = useState<Feed[]>([]);
  const [items, setItems] = useState<ItemWithState[]>([]);
  const [selectedFeedId, setSelectedFeedId] = useState<string | null>(null);
  const [selectedItem, setSelectedItem] = useState<ItemWithState | null>(null);
  const [onlyUnread, setOnlyUnread] = useState(false);
  const [onlyStarred, setOnlyStarred] = useState(false);
  const [showDetail, setShowDetail] = useState(false);

  const loadFeeds = useCallback(async () => {
    try {
      setFeeds(await getFeeds());
    } catch (e) {
      console.error("Failed to load feeds:", e);
    }
  }, []);

  const loadItems = useCallback(async () => {
    try {
      const result = await getItems(selectedFeedId ?? undefined, {
        unreadOnly: onlyUnread,
        starredOnly: onlyStarred,
        limit: 200,
        offset: 0,
      });
      setItems(result);
    } catch (e) {
      console.error("Failed to load items:", e);
    }
  }, [selectedFeedId, onlyUnread, onlyStarred]);

  useEffect(() => {
    loadFeeds();
  }, [loadFeeds]);

  useEffect(() => {
    loadItems();
  }, [loadItems]);

  const handleSelectItem = (item: ItemWithState) => {
    setSelectedItem(item);
    setShowDetail(true);
  };

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      <Sidebar
        feeds={feeds}
        selectedFeedId={selectedFeedId}
        onSelectFeed={setSelectedFeedId}
        onFeedsChanged={loadFeeds}
      />
      <div className="flex flex-1 flex-col overflow-hidden">
        <TopBar
          onlyUnread={onlyUnread}
          onlyStarred={onlyStarred}
          onToggleUnread={() => setOnlyUnread(!onlyUnread)}
          onToggleStarred={() => setOnlyStarred(!onlyStarred)}
          onRefresh={loadItems}
          selectedFeed={feeds.find((f) => f.id === selectedFeedId) ?? null}
        />
        <div className="flex flex-1 overflow-hidden">
          <div className={`${showDetail ? "w-1/2" : "w-full"} flex flex-col overflow-hidden border-r`}>
            <ItemList
              items={items}
              selectedItemId={selectedItem?.id ?? null}
              onSelectItem={handleSelectItem}
              onItemsChanged={loadItems}
            />
          </div>
          {showDetail && (
            <div className="w-1/2 flex flex-col overflow-hidden">
              <ItemDetail
                item={selectedItem}
                onClose={() => setShowDetail(false)}
                onItemsChanged={loadItems}
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
