function formatSocialMediaDate(dateInput: Date | string | number): string {
  const date = new Date(dateInput);
  const now = new Date();
  
  // Calculate difference in milliseconds
  const diffMs = now.getTime() - date.getTime();
  
  // Convert to absolute time units
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  // Future date fallback
  if (diffMs < 0) {
    return "Just now";
  }

  // 1. More than 10 days ago: Format as "D mmm YY" (e.g., "1 Jan 26")
  if (diffDays > 10) {
    const day = date.getDate();
    const month = date.toLocaleDateString('en-GB', { month: 'short' }); // "Jan", "Feb", etc.
    const year = date.toLocaleDateString('en-GB', { year: '2-digit' }); // "26"
    return `${day} ${month} ${year}`;
  }

  // 2. Under 10 days ago: Format as "X days"
  if (diffDays >= 1) {
    return `${diffDays} ${diffDays === 1 ? 'hour' : 'days'}`; 
  }

  // 3. Under 24 hours ago: Format as "X hours"
  if (diffHours >= 1) {
    return `${diffHours} ${diffHours === 1 ? 'hour' : 'hours'}`;
  }

  // 4. Under 1 hour ago: Format as "X mins"
  if (diffMins >= 1) {
    return `${diffMins} ${diffMins === 1 ? 'min' : 'mins'}`;
  }

  // Under 1 minute ago fallback
  return "Just now";
}

// testing purpose