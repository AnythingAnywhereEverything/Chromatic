export async function fetchOpenGraphData(url: string): Promise<any> {
    const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}v2/opengraph`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ url }),
    });
    if (!response.ok) {
        throw new Error('Failed to fetch OpenGraph data');
    }
    return response.json();
}