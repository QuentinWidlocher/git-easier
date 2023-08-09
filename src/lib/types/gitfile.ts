const statuses = [
  "current",
  "index_new",
  "index_modified",
  "index_deleted",
  "index_renamed",
  "index_typechange",
  "wt_new",
  "wt_modified",
  "wt_deleted",
  "wt_typechange",
  "wt_renamed",
  "ignored",
  "conflicted",
] as const;

export type Status = typeof statuses[number];

export function isStaged(file: GitFile) {
  return file.statuses.some(status => status.includes('index'));
}

export type GitFile = {
  path: string;
  statuses: Status[];
}
