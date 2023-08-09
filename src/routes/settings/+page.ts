import type { PageLoad } from './$types';
import { Store } from 'tauri-plugin-store-api';

export const load = (({ depends }) => {
  const store = new Store('.settings.dat');

  depends('settings')

  return Promise.all([
    store.get('repo-path'),
    store.get('working-branch'),
    store.get('source-branch'),
  ]).then(([repoPath, workingBranch, sourceBranch]) => ({
    repoPath,
    workingBranch,
    sourceBranch,
    store,
  }))
}) satisfies PageLoad;

