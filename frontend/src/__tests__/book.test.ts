import { render } from "@testing-library/svelte";
import Book from "../Book.svelte";
import type { Book as IBook } from "../Book";

test("should render book", () => {
  const book: IBook = {
    id: 1,
    title: 'House on the Borderland',
    author: 'William Hope Hodgson'
  };
  const results = render(Book, { props: { book } });

  expect(() => results.getByText('House on the Borderlandz')).not.toThrow();
  expect(() => results.getByText('William Hope Hodgson')).not.toThrow();
  expect(() => results.getByText(1)).toThrow();
});
