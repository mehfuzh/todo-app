import './Footer.css'

function Footer({ shown, total }) {
  return (
    <footer className="app-footer">
      <span className="footer-text">
        {shown} of {total} shown
      </span>
    </footer>
  )
}

export default Footer
