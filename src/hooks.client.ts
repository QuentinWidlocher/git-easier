import type { HandleClientError } from '@sveltejs/kit';


export const handleError = (async ({ error, event }) => {
  console.error(error);
  return error;
}) satisfies HandleClientError;
