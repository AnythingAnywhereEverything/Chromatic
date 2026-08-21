const getIdColor = (userId: string) => {
    let id = BigInt(userId || 0n);

    id = ((id >> 30n) ^ id) * 0xbf58476d1ce4e5b9n;
    id = ((id >> 27n) ^ id) * 0x94d049bb133111ebn;
    id = (id >> 31n) ^ id;

    const hue = Number(id % 360n);
    const absoluteHue = Math.abs(hue);

    return `hsl(${absoluteHue}, 60%, 45%)`;
};

export default getIdColor;

