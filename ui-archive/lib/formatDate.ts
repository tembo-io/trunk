import { formatDistanceToNow, parseISO, format } from "date-fns";

export function formatDateString(date: String | undefined) {
  try {
    if (!date) {
      return "";
    }
    const formatted = format(new Date(date.split(".")[0].replace(/-/g, "/")), "yyyy-MM-dd HH:mm");
    const isoParse = parseISO(`${formatted}Z`);
    const formattedDate = formatDistanceToNow(isoParse, { addSuffix: true });
    return formattedDate;
  } catch (error) {
    return "";
  }
}
