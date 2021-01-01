export interface PostSummary {
	title: string;
	id: number;
	user: string;
	date: string;
	url: string;
}

export interface PostType extends PostSummary {
	html_content: string;
}
