import Image from "next/image";

type AvatarProps = {
  src?: string | null;
  size?: number;
  fill?: boolean;
  className?: string;
};

export default function Avatar({
  src,
  size = 200,
  fill = false,
  className,
}: AvatarProps) {

  const isAbsolute =
    src?.startsWith("http") || src?.startsWith("blob:");

  const finalSrc = src
    ? isAbsolute
      ? src
      : `/cdn/${src}`
    : "/default/default_profile.jpg";

  if (fill) {
    return (
      <div className={className} style={{ position: "relative" }}>
        <Image
          src={finalSrc}
          alt="User profile"
          fill
          style={{ objectFit: "cover" }}
        />
      </div>
    );
  }

  return (
    <Image
      src={finalSrc}
      alt="User profile"
      width={size}
      height={size}
    />
  );
}