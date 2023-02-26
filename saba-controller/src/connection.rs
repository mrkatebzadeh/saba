/* connection.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Connection {
    pub src: String,
    pub dst: String,
}

impl Connection {
    #[allow(dead_code)]
    pub fn new(src: String, dst: String) -> Self {
        Connection { src, dst }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let connection = Connection::new("a".to_string(), "b".to_string());
        assert_eq!(connection.src, "a");
        assert_eq!(connection.dst, "b");
    }

    #[test]
    fn test_connection_eq() {
        let connection1 = Connection::new("a".to_string(), "b".to_string());
        let connection2 = Connection::new("a".to_string(), "b".to_string());
        assert_eq!(connection1, connection2);
    }

    #[test]
    fn test_connection_neq() {
        let connection1 = Connection::new("a".to_string(), "b".to_string());
        let connection2 = Connection::new("a".to_string(), "c".to_string());
        assert_ne!(connection1, connection2);
    }
}

/* connection.rs ends here */
