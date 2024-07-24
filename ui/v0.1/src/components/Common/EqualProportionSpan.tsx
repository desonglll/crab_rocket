import "./EqualProportionSpan.scss";

//等比例组件
export function EqualProportionSpan({
  w,
  h,
  children,
}: {
  w: number;
  h: number;
  children: any;
}) {
  const style = {
    marginTop: w && h ? `${(h / w) * 100}%` : "100%",
  };
  return (
    <div className="equal-proportion">
      <div className="equal-proportion-body">{children}</div>
      <div style={style} />
    </div>
  );
}
