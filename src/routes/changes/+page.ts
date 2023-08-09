import { getChangedFiles } from '$lib/services/back';
import { isStaged, type GitFile } from '$lib/types/gitfile';
import type { PageLoad } from './$types';

export const load = (({ depends }) => {
  depends('changes:list')
  console.log('changes list loading');
  return getChangedFiles().then((files) => {
    return files.reduce(
      (acc, file) => {
        if (isStaged(file)) {
          acc.staged.push(file);
        } else {
          acc.unstaged.push(file);
        }
        return acc;
      },
      { unstaged: [], staged: [] } as { unstaged: Array<GitFile>, staged: Array<GitFile> }
    );
  });
}) satisfies PageLoad;
