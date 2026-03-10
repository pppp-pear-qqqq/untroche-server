export function swap(a: Element, b: Element) {
	if (a !== b) {
		const next = a.nextElementSibling;
		const parent = next !== null ? next.parentElement! : a.parentElement!;
		if (next === b) {
			parent.insertBefore(b, a);
		} else {
			b.parentElement!.insertBefore(a, b);
			parent.insertBefore(b, next);
		}
	}
}
