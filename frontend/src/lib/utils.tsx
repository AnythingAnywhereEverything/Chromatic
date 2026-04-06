import { useEffect, useRef, useState } from "react";

export function cn(...classes: (string | undefined | false)[]): string {
    return classes.filter(Boolean).join(" ")
}

export function timeAgo(date: string | Date): string {
    let parsed: Date
    if (typeof date === "string") {
        parsed = new Date(date.endsWith("Z") ? date : date + "Z")
    } else {
        parsed = date
    }

    if (isNaN(parsed.getTime())) return ""

    const diff = Math.floor((Date.now() - parsed.getTime()) / 1000)

    const minute = 60
    const hour = 60 * minute
    const day = 24 * hour
    const month = 30 * day
    const year = 365 * day

    if (diff < 10) return "just now"
    if (diff < minute) return `${diff}s ago`
    if (diff < hour) return `${Math.floor(diff / minute)}m ago`
    if (diff < day) return `${Math.floor(diff / hour)}h ago`
    if (diff < month) return `${Math.floor(diff / day)}d ago`
    if (diff < year) return `${Math.floor(diff / month)}mo ago`

    return `${Math.floor(diff / year)}y ago`
}

export const useGridColumnCount = () => {
    const containerRef = useRef<HTMLUListElement | null>(null);
    const [columnCount, setColumnCount] = useState(0);

    useEffect(() => {
        if (!containerRef.current) return;

        const el = containerRef.current;

        const calculateColumns = () => {
            const computedStyle = window.getComputedStyle(el);
            const columns = computedStyle
                .gridTemplateColumns
                .split(" ")
                .filter(Boolean).length;

            setColumnCount(columns);
        };

        calculateColumns();

        const observer = new ResizeObserver(calculateColumns);
        observer.observe(el);

        return () => observer.disconnect();
    }, []);

    return { containerRef, columnCount };
};

export const formatLargeNumber = (num:number|bigint) => {
    return new Intl.NumberFormat('en-US', {
        notation: 'compact',
        maximumFractionDigits: 1
    }).format(num);
}

export const ratingStars = (itemrating:number) => {
    const stars = [];
    for (let i = 0; i < Math.floor(itemrating); i++) {
        stars.push("");
    }
    if (itemrating % 1 > 0) {
        stars.push("");
    }
    for (let i = stars.length; i < 5; i++) {
        stars.push("");
    }
    return stars.join("");
}

export function formatPhoneByCountry(digits: string, countryCode: string) {
    const numbers = digits.replace(/\D/g, "");

    if (!countryCode) return numbers;

    if (countryCode === "1") {
        const area = numbers.slice(0, 3);
        const mid = numbers.slice(3, 6);
        const last = numbers.slice(6, 10);

        let formatted = "+1";
        if (area) formatted += ` (${area}`;
        if (area.length === 3) formatted += `)`;
        if (mid) formatted += ` ${mid}`;
        if (last) formatted += `-${last}`;
        return formatted;
    }

    return `+${countryCode} ${numbers}`;
}

export function formatDateTime(time:string){
    const standardizedString = time.replace(' ', 'T') + 'Z';
    const localDate = new Date(standardizedString);
    
    const datePart = localDate.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });
    const timePart = localDate.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    return (`${datePart} at ${timePart}`);
}
