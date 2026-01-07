
export interface Note {
  id: string;
  title: string;
  content: string;
  type: "note";
  links: string[];
  status: "inbox" | "active" | "archived";
  created_at: number;
  updated_at: number;
}
