import { useState } from "react";

export const useUndoRedo = (initialValue:string) => {
    const [history, setHistory] = useState<(string)[]>([initialValue]);
    const [currentIndex, setCurrentIndex] = useState(0);

    const set = (value: string) => {
        const current = history[currentIndex];
        
        const blankState = current === initialValue && value.length === initialValue.length + 1;

        if (!value.endsWith(" ")&& !blankState) {
            setHistory(prev => {
                const next = [...prev.slice(0, currentIndex), value, ...prev.slice(currentIndex + 1)];
                return next;
            });
            return;
        }

        const newHistory = [...history.slice(0, currentIndex + 1), value];
        setHistory(newHistory);
        setCurrentIndex(newHistory.length - 1);
    };

    const undo = () => {
        if (currentIndex <= 0) return;
        setCurrentIndex(prev => prev - 1);
    };

    const redo = () => {
        if (currentIndex >= history.length - 1) return;
        setCurrentIndex(prev => prev + 1);
    };

    return {
        value: history[currentIndex],
        set,
        undo,
        redo,
        canUndo: currentIndex > 0,
        canRedo: currentIndex < history.length - 1,
    };
}