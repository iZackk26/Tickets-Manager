import React, { Component } from "react";
import { Link } from "react-router-dom";
import { FaHome } from "react-icons/fa"; // Ícono de inicio
import { AiOutlineRight } from "react-icons/ai"; // Separador

class BreadcrumbsWrapper extends Component {
  getPathnames() {
    const pathname = window.location.pathname;
    return pathname.split("/").filter((x) => x);
  }

  render() {
    const pathnames = this.getPathnames();
    const normalizedPath = window.location.pathname.toLowerCase().replace(/\/+$/, "");

    return (
      <nav aria-label="Breadcrumb" className="w-full p-4">
        <div className="flex space-x-2 items-center justify-start">
          {/* Ícono de inicio */}
          <Link
            to="/"
            className="opacity-60 flex items-center space-x-1 hover:opacity-100 transition-opacity"
            aria-label="Home"
          >
            <FaHome className="text-gray-700" />
          </Link>

          {/* Solo mostrar "Seats" cuando la ruta es /seats/:zone/:category */}
          {normalizedPath.startsWith("/seats/") && pathnames.length === 3 ? (
            <>
              <AiOutlineRight className="text-gray-500" />
              <span className="font-semibold text-gray-700">Seats</span>
            </>
          ) : (
            // Mostrar breadcrumbs estándar para otras rutas
            pathnames.map((value, index) => {
              const to = `/${pathnames.slice(0, index + 1).join("/")}`;
              const label = value.charAt(0).toUpperCase() + value.slice(1);
              const isLast = index === pathnames.length - 1;

              return (
                <div key={to} className="flex items-center space-x-2">
                  <AiOutlineRight className="text-gray-500" />
                  {isLast ? (
                    <span className="font-semibold text-gray-700">{label}</span>
                  ) : (
                    <Link
                      to={to}
                      className="opacity-60 hover:opacity-100 transition-opacity text-gray-700"
                    >
                      {label}
                    </Link>
                  )}
                </div>
              );
            })
          )}
        </div>
      </nav>
    );
  }
}

export default BreadcrumbsWrapper;
