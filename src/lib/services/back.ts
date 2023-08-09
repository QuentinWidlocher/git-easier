import type { GitFile } from '$lib/types/gitfile';
import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';

type Commands = {
  "get_changed_files": [never, Array<GitFile>]
  "move_file": [{
    path: string,
    action: 'stage' | 'unstage'
  }, void]
  "move_all_files": [{
    action: 'stage' | 'unstage'
  }, void]
  "publish": [{ message?: string | undefined }, void]
};

type CommandName = keyof Commands;
type CommandArgs<T extends CommandName> = Commands[T][0];
type CommandReturnType<T extends CommandName> = Commands[T][1];

function typedInvoke<T extends CommandName>(command: T,) {
  return function(args: CommandArgs<T>): Promise<CommandReturnType<T>> {
    return invoke(command, args as unknown as InvokeArgs);
  }
}

function typedInvokeWithoutArgs<T extends CommandName>(command: T,) {
  return function(): Promise<CommandReturnType<T>> {
    return invoke(command);
  }
}


export const getChangedFiles = typedInvokeWithoutArgs("get_changed_files");
export const moveFile = typedInvoke("move_file");
export const moveAllFile = typedInvoke("move_all_files");
export const publish = typedInvoke("publish");
