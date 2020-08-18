describe('It Renders ', () => {
	it('Has default dark theme', () => {
		cy.visit('http://localhost:8000/');
		cy.get('#main').should('have.class', 'dark');
	});
});
