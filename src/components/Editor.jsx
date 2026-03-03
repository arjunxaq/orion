export default function Editor() {
  const [content, setContent] = useState("")

  return (
    <div className="editor">
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="Start writing..."
      />
    </div>
  )
}